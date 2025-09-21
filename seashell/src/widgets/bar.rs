mod imp {
    use glib::types::StaticTypeExt;
    use gtk::{prelude::*, subclass::prelude::*};
    use gtk4_layer_shell::{Edge, Layer, LayerShell};

    use crate::widgets::{Clock, Tray, auto_hide::AutoHide};

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(string = r#"
    using Gtk 4.0;

    template $Bar : ApplicationWindow {
        $AutoHide {
            child: Box {
                styles ["bar"]

                $Tray {}
                $Clock {}
            };
        }
    }
    "#)]
    pub struct Bar {}

    #[glib::object_subclass]
    impl ObjectSubclass for Bar {
        const NAME: &'static str = "Bar";
        type Type = super::Bar;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Tray::ensure_type();
            AutoHide::ensure_type();
            Clock::ensure_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Bar {
        fn constructed(&self) {
            self.parent_constructed();

            let window = self.obj();
            window.init_layer_shell();
            window.set_layer(Layer::Overlay);
            window.set_anchor(Edge::Top, true);
            window.set_default_size(1, 1);
            window.queue_resize();
        }
    }

    impl WidgetImpl for Bar {}

    impl WindowImpl for Bar {}

    impl ApplicationWindowImpl for Bar {}
}

impl Bar {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}

glib::wrapper! {
    pub struct Bar(ObjectSubclass<imp::Bar>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
