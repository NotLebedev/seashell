mod imp {
    use std::pin::pin;

    use dbus_upower::UPower;
    use futures::StreamExt;
    use glib::clone;
    use gtk::subclass::prelude::*;
    use log::{error, info};

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(string = r#"
    using Gtk 4.0;

    template $Battery: Box {
        Label label {
            label: "0%";
        }
    }
    "#)]
    pub struct Battery {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
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
                #[weak(rename_to = label)]
                self.label,
                async move {
                    let Ok(upower) = UPower::new().await else {
                        error!("Could not connect to UPower bus.");
                        return;
                    };

                    let Ok(display_device) = upower.get_display_device().await else {
                        error!("Could not connect to device.");
                        return;
                    };

                    let mut percentage_stream = pin!(display_device.listen_percentage().await);

                    while let Some(percentage) = percentage_stream.next().await {
                        if let Ok(percentage) = percentage {
                            label.set_label(&format!("{}%", percentage.round()));
                        }
                    }
                }
            ));
        }
    }

    impl WidgetImpl for Battery {}

    impl BoxImpl for Battery {}
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
