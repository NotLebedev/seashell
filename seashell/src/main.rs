mod monitors;
mod widgets;

use gio::{ApplicationFlags, prelude::*};
use log::error;

use crate::monitors::open_on_main_monitor;

fn activate(application: &gtk::Application) {
    env_logger::init();
    dbus_tray::start_server();

    let hold = application.hold();
    Box::leak(Box::new(hold));

    open_on_main_monitor(application, widgets::Bar::new);
}

fn main() {
    let app = gtk::Application::new(Some("org.notlebedev.seashell"), ApplicationFlags::default());

    app.connect_startup(|_| {
        set_gtk_settings();
        load_css();
    });
    app.connect_activate(activate);

    app.run_with_args(&Vec::<String>::new());
}

pub fn load_css() {
    const CSS_SOURCE: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
    if let Some(display) = gdk::Display::default() {
        let css = gtk::CssProvider::new();
        css.load_from_string(CSS_SOURCE);
        gtk::style_context_add_provider_for_display(
            &display,
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        error!("Failed to load css, could not get gdk::Display");
    }
}

pub fn set_gtk_settings() {
    let Some(settings) = gtk::Settings::default() else {
        error!("Failed to get GTK settings");
        return;
    };

    settings.set_property("gtk-cursor-aspect-ratio", 0.04);
    settings.set_property("gtk-font-name", "Inter 12");
}
