mod imp {
    use glib::clone;
    use gtk::subclass::prelude::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(string = r#"
    using Gtk 4.0;

    template $Clock: Box {
        orientation: horizontal;
        Label clock {
            label: "00:00";
            halign: end;
        }
    }
    "#)]
    pub struct Clock {
        #[template_child]
        pub clock: gtk::TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Clock {
        const NAME: &'static str = "Clock";
        type Type = super::Clock;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Clock {
        fn constructed(&self) {
            self.parent_constructed();

            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    loop {
                        let Ok(text) =
                            glib::DateTime::now_local().and_then(|dt| dt.format("%H:%M"))
                        else {
                            continue;
                        };
                        this.clock.set_text(&text);

                        glib::timeout_future_seconds(1).await;
                    }
                }
            ));
        }
    }

    impl WidgetImpl for Clock {}
    impl BoxImpl for Clock {}
}

glib::wrapper! {
    pub struct Clock(ObjectSubclass<imp::Clock>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Clock {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
