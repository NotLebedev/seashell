use std::pin::pin;

use dbus_tray::{TrayItem, TrayServer};
use futures::StreamExt;
use glib::clone;
use gtk::{prelude::*, subclass::prelude::*};
use log::error;

mod imp {
    use std::cell::RefCell;

    use glib::clone;
    use gtk::{prelude::*, subclass::prelude::*};

    use crate::widgets::auto_hide::AutoHide;

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::Tray)]
    #[template(string = r#"
    using Gtk 4.0;

    template $Tray: Box {
        styles ["tray"]

        orientation: horizontal;
        spacing: 10;
    }
    "#)]
    pub struct Tray {
        #[property(set, nullable, name = "auto-hide")]
        pub auto_hide: RefCell<Option<AutoHide>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Tray {
        const NAME: &'static str = "Tray";
        type Type = super::Tray;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for Tray {
        fn constructed(&self) {
            self.parent_constructed();

            glib::spawn_future_local(clone!(
                #[weak(rename_to = obj)]
                self.obj(),
                async move {
                    obj.listen_tray().await;
                }
            ));
        }
    }

    impl WidgetImpl for Tray {}

    impl BoxImpl for Tray {}
}

glib::wrapper! {
    pub struct Tray(ObjectSubclass<imp::Tray>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Tray {
    pub fn new() -> Self {
        glib::Object::new()
    }

    async fn listen_tray(&self) {
        let Ok(tray_server) = TrayServer::new().await else {
            error!("Failed to connect to tray server");
            return;
        };
        let Ok(stream) = tray_server.listen_items_updated().await else {
            error!("Failed to start listening to item_updated events");
            return;
        };
        let mut stream = pin!(stream);

        loop {
            let child_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
            if let Ok(items) = tray_server.items().await
                && let Ok(icons) = self.load_items(items).await
            {
                for icon in &icons {
                    child_box.append(icon);
                }
            }
            self.append(&child_box);

            let Some(()) = stream.next().await else {
                break;
            };

            self.remove(&child_box);
        }
    }

    async fn load_items(&self, items: Vec<TrayItem>) -> anyhow::Result<Vec<gtk::Widget>> {
        let mut res = Vec::new();
        for item in items {
            res.push(self.tray_item(item).await.into());
        }

        Ok(res)
    }

    async fn tray_item(&self, item: TrayItem) -> gtk::MenuButton {
        let icon = gtk::Image::new();
        if let Ok(gicon) = item.gicon().await {
            icon.set_from_gicon(&gicon);
        }

        let update_icon_fut = glib::spawn_future_local(clone!(
            #[weak]
            icon,
            #[strong]
            item,
            async move {
                let Ok(stream) = item.listen_gicon_updated().await else {
                    return;
                };
                let mut stream = pin!(stream);
                while let Some(()) = stream.next().await {
                    if let Ok(gicon) = item.gicon().await {
                        icon.set_from_gicon(&gicon);
                    }
                }
            }
        ));

        let menu_button = gtk::MenuButton::new();
        menu_button.set_child(Some(&icon));
        menu_button.set_css_classes(&["item"]);
        if let Ok(layout) = item.layout().await {
            menu_button.set_menu_model(Some(&layout.as_menu_model()));
            menu_button.insert_action_group("dbusmenu", Some(&layout.as_action_group(&item)));
        }
        if let Some(popover) = menu_button.popover() {
            popover.set_has_arrow(false);
            // Compensate for no arrow in layout
            popover.set_offset(0, 16);
        }

        let update_popover_fut = glib::spawn_future_local(clone!(
            #[weak]
            menu_button,
            #[strong]
            item,
            async move {
                let Ok(stream) = item.listen_layout_updated().await else {
                    return;
                };
                let mut stream = pin!(stream);
                while let Some(()) = stream.next().await {
                    if let Ok(layout) = item.layout().await {
                        menu_button.set_menu_model(Some(&layout.as_menu_model()));
                        menu_button
                            .insert_action_group("dbusmenu", Some(&layout.as_action_group(&item)));
                        // Because popup was recreated with new menu model
                        // we need to hide arrow again
                        if let Some(popover) = menu_button.popover() {
                            popover.set_has_arrow(false);
                            // Compensate for no arrow in layout
                            popover.set_offset(0, 16);
                        }
                    }
                }
            }
        ));

        // No, "Broken accounting of active state for widget" is not because
        // of this. It's a gtk bug and a harmless one it seems. See
        // https://gitlab.gnome.org/GNOME/gtk/-/blob/af64eb18ec9f3a9c0267b9eba44fb5fff71d0056/gtk/gtkwidget.c#L13379
        let click_controller = gtk::GestureClick::new();
        click_controller.set_button(0);
        click_controller.connect_pressed(clone!(
            #[weak]
            menu_button,
            #[strong]
            item,
            move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);

                let button = gesture.current_button();
                let is_menu = item.is_menu();

                if !is_menu && button == 1 {
                    glib::spawn_future_local(clone!(
                        #[strong]
                        item,
                        async move { item.activate().await }
                    ));
                } else if button == 1 || button == 3 {
                    menu_button.popup();
                }
            }
        ));

        menu_button.add_controller(click_controller);

        menu_button.connect_active_notify(clone!(
            #[weak(rename_to = tray)]
            self,
            move |menu_button| {
                if let Some(auto_hide) = &*tray.imp().auto_hide.borrow() {
                    if menu_button.is_active() {
                        auto_hide.reveal();
                    } else {
                        auto_hide.hide();
                    }
                }
            }
        ));

        // Stop waiting for updates when item
        // is removed from stack
        icon.connect_destroy(move |_| {
            update_icon_fut.abort();
        });
        menu_button.connect_destroy(move |_| {
            update_popover_fut.abort();
        });

        menu_button
    }
}
