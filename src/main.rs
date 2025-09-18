use gtk::gio;
use gtk::gio::DBusMenuModel;
use gtk::gio::prelude::*;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::prelude::*;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use log::info;

use crate::tray::dbus::StatusNotifierItemProxy;

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
    let b = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    window.set_child(Some(&b));

    glib::spawn_future_local(clone!(
        #[weak]
        b,
        async move {
            let menus = status_notifier().await;

            for menu in menus {
                let mm = gtk::MenuButton::new();
                mm.set_menu_model(Some(&menu));
                b.append(&mm);
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

async fn status_notifier() -> Vec<DBusMenuModel> {
    let conn = tray::dbus::StatusNotifierWatcher::start_server()
        .await
        .unwrap();

    let proxy = tray::dbus::StatusNotifierWatcherProxy::new(&conn)
        .await
        .unwrap();

    let gio_conn = gio::bus_get_future(gio::BusType::Session).await.unwrap();

    let items = proxy.registered_status_notifier_items().await.unwrap();

    let mut models = Vec::new();

    for name in items {
        let (dest, path) = if let Some(idx) = name.find('/') {
            (&name[..idx], &name[idx..])
        } else {
            (name.as_ref(), "/StatusNotifierItem")
        };
        info!("{dest}, {path}");
        let item = StatusNotifierItemProxy::new(&conn, dest, path)
            .await
            .unwrap();
        let Ok(menu) = item.menu().await else {
            continue;
        };
        info!("{}", menu);

        let client = dbusmenu_glib::Client::new(dest, menu);
        client.menu();
    }

    models
}
