use std::pin::pin;

use futures::StreamExt;
use gtk::{
    gio::{self, prelude::*},
    glib::{self, clone},
    prelude::*,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use log::{error, info};

use crate::tray::{Layout, TrayItem, TrayServer};

mod tray;

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application) {
    env_logger::init();
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Overlay);

    // Push other windows out of the way
    // window.auto_exclusive_zone_enable();

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 40);
    window.set_margin(Edge::Right, 40);
    window.set_margin(Edge::Top, 20);
    window.set_anchor(Edge::Top, true);

    // Set up a widget
    let tray_container = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    window.set_child(Some(&tray_container));

    glib::spawn_future_local(clone!(
        #[weak]
        tray_container,
        async move {
            let Ok(()) = tray::start_server().await else {
                error!("Failed to start tray backend server");
                return;
            };

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
                if let Ok(items) = tray_server.items().await {
                    if let Ok(icons) = load_items(items).await {
                        for icon in &icons {
                            child_box.append(icon);
                        }
                    };
                }
                tray_container.append(&child_box);

                let Some(()) = stream.next().await else {
                    break;
                };
                info!("List updated");

                tray_container.remove(&child_box);
            }
        }
    ));

    window.present();
}

fn main() {
    let app = gtk::Application::new(Some("org.notlebedev.seashell"), Default::default());

    app.connect_activate(|app| {
        activate(app);
    });

    app.run_with_args(&Vec::<String>::new());
}

async fn load_items(items: Vec<TrayItem>) -> anyhow::Result<Vec<gtk::Widget>> {
    let mut res = Vec::new();
    for item in items.into_iter() {
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
        if let Ok(layout) = item.layout().await {
            menu_button.set_menu_model(Some(&layout.as_menu_model()));
            menu_button.insert_action_group("dbusmenu", Some(&layout.as_action_group()));
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
                            .insert_action_group("dbusmenu", Some(&layout.as_action_group()));
                    }
                }
            }
        ));

        icon.connect_destroy(move |_| {
            // Stop waiting for updates when item
            // is removed from stack
            update_icon_fut.abort();
            update_popover_fut.abort();
            info!("aborted");
        });

        res.push(menu_button.into());
    }

    Ok(res)
}
