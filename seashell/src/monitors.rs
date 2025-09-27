use std::{cell::RefCell, rc::Rc};

use gdk::prelude::*;
use glib::clone;
use gtk::prelude::{GtkApplicationExt, GtkWindowExt};

fn select_monitor(list: &gio::ListModel) -> Option<gdk::Monitor> {
    list.item(0)
        .and_then(|obj| obj.dynamic_cast::<gdk::Monitor>().ok())
}

fn install_window<R, F>(
    current_window: &Rc<RefCell<Option<gtk::ApplicationWindow>>>,
    monitor: Option<&gdk::Monitor>,
    application: &gtk::Application,
    window_builder: &F,
) where
    R: IsA<gtk::ApplicationWindow> + gtk4_layer_shell::LayerShell,
    F: Fn(&gtk::Application) -> R + 'static,
{
    if let Some(old_window) = current_window.replace(None) {
        application.remove_window(&old_window);
    }

    if let Some(monitor) = monitor {
        let window = window_builder(application);
        window.set_monitor(Some(monitor));
        window.present();
        current_window.replace(Some(window.into()));
    }
}

pub fn open_on_main_monitor<R, F>(application: &gtk::Application, window_builder: F)
where
    R: IsA<gtk::ApplicationWindow> + gtk4_layer_shell::LayerShell,
    F: Fn(&gtk::Application) -> R + 'static,
{
    let current_window = Rc::new(RefCell::new(None::<gtk::ApplicationWindow>));

    if let Some(display) = gdk::Display::default() {
        install_window(
            &current_window,
            select_monitor(&display.monitors()).as_ref(),
            application,
            &window_builder,
        );

        display.monitors().connect_items_changed(clone!(
            #[weak]
            application,
            #[strong]
            current_window,
            move |list, _, _, _| {
                install_window(
                    &current_window,
                    select_monitor(list).as_ref(),
                    &application,
                    &window_builder,
                );
            }
        ));
    }
}
