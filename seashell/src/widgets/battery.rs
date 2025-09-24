mod imp {
    use std::pin::pin;

    use dbus_upower::{Device, UPower};
    use futures::StreamExt;
    use glib::clone;
    use gtk::{prelude::*, subclass::prelude::*};
    use log::error;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(string = r#"
    using Gtk 4.0;

    template $Battery: Box {
        orientation: horizontal;
        styles ["battery"]

        Image icon {}

        Label label {
            label: "0%";
        }
    }
    "#)]
    pub struct Battery {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Battery {
        const NAME: &'static str = "Battery";
        type Type = super::Battery;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Battery {
        fn constructed(&self) {
            self.parent_constructed();

            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    let Ok(upower) = UPower::new().await else {
                        error!("Could not connect to UPower bus.");
                        return;
                    };

                    let Ok(display_device) = upower.get_display_device().await else {
                        error!("Could not connect to device.");
                        return;
                    };

                    if let Ok(false) | Err(_) = display_device.is_present().await {
                        this.obj().set_visible(false);
                        return;
                    }

                    glib::spawn_future_local(clone!(
                        #[weak]
                        this,
                        #[strong]
                        display_device,
                        async move { this.monitor_percentage(display_device).await }
                    ));

                    glib::spawn_future_local(clone!(
                        #[weak]
                        this,
                        #[strong]
                        display_device,
                        async move { this.monitor_icon(display_device).await }
                    ));
                }
            ));
        }
    }

    impl WidgetImpl for Battery {}

    impl BoxImpl for Battery {}

    impl Battery {
        async fn monitor_percentage(&self, device: Device) {
            let mut percentage_stream = pin!(device.listen_percentage().await);

            while let Some(percentage) = percentage_stream.next().await {
                if let Ok(percentage) = percentage {
                    self.label.set_label(&format!("{}%", percentage.round()));
                }
            }
        }

        async fn monitor_icon(&self, device: Device) {
            let mut icon_stream = pin!(device.icon_name().await);

            while let Some(icon) = icon_stream.next().await {
                if let Ok(icon) = icon
                    && let Ok(icon) = gio::Icon::for_string(icon.as_str())
                {
                    self.icon.set_from_gicon(&icon);
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct Battery(ObjectSubclass<imp::Battery>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Battery {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
