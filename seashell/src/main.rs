mod widgets;

use gio::{ApplicationFlags, prelude::*};
use gtk::prelude::*;

fn activate(application: &gtk::Application) {
    env_logger::init();
    dbus_tray::start_server();

    let bar = widgets::Bar::new(application);
    bar.present();
}

fn main() {
    let app = gtk::Application::new(Some("org.notlebedev.seashell"), ApplicationFlags::default());

    app.connect_activate(activate);

    app.run_with_args(&Vec::<String>::new());
}
