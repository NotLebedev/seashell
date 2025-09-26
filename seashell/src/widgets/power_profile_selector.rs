mod imp {
    use std::pin::pin;

    use dbus_upower::{PowerProfile, PowerProfiles};
    use futures::StreamExt;
    use glib::clone;
    use gtk::{prelude::ToggleButtonExt, subclass::prelude::*};

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(string = r#"
    using Gtk 4.0;

    template $PowerProfileSelector: Box {
        styles ["toggleGroup"]
        orientation: horizontal;
        hexpand: true;
        homogeneous: true;

        ToggleButton saver {
            clicked => $set_saver() swapped;

            child: Image {
                icon-name: "power-profile-power-saver-symbolic";
                icon-size: large;
            };
        }

        ToggleButton balanced {
            clicked => $set_balanced() swapped;
            group: saver;

            child: Image {
                icon-name: "power-profile-balanced-symbolic";
                icon-size: large;
            };
        }

        ToggleButton performance {
            clicked => $set_performance() swapped;
            group: saver;

            child: Image {
                icon-name: "power-profile-performance-symbolic";
                icon-size: large;
            };
        }
    }
    "#)]
    pub struct PowerProfileSelector {
        #[template_child]
        saver: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        balanced: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        performance: TemplateChild<gtk::ToggleButton>,
        power_profiles: async_once_cell::OnceCell<PowerProfiles>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PowerProfileSelector {
        const NAME: &'static str = "PowerProfileSelector";
        type Type = super::PowerProfileSelector;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PowerProfileSelector {
        fn constructed(&self) {
            self.parent_constructed();

            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    let Ok(power_profiles) = this.get_power_profiles().await else {
                        return;
                    };

                    let mut current_profile = pin!(power_profiles.listen_active_profile().await);

                    while let Some(current_profile) = current_profile.next().await {
                        if let Ok(current_profile) = current_profile {
                            match current_profile {
                                PowerProfile::PowerSaver => this.saver.set_active(true),
                                PowerProfile::Balanced => this.balanced.set_active(true),
                                PowerProfile::Performance => this.performance.set_active(true),
                            }
                        }
                    }
                }
            ));
        }
    }

    impl WidgetImpl for PowerProfileSelector {}

    impl BoxImpl for PowerProfileSelector {}

    #[gtk::template_callbacks]
    impl PowerProfileSelector {
        async fn get_power_profiles(&self) -> anyhow::Result<&PowerProfiles> {
            self.power_profiles
                .get_or_try_init(async { PowerProfiles::new().await })
                .await
        }

        #[template_callback]
        fn set_saver(&self, _button: gtk::ToggleButton) {
            self.set_profile(PowerProfile::PowerSaver);
        }

        #[template_callback]
        fn set_balanced(&self, _button: gtk::ToggleButton) {
            self.set_profile(PowerProfile::Balanced);
        }

        #[template_callback]
        fn set_performance(&self, _button: gtk::ToggleButton) {
            self.set_profile(PowerProfile::Performance);
        }

        fn set_profile(&self, profile: PowerProfile) {
            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    let Ok(power_profiles) = this.get_power_profiles().await else {
                        return;
                    };

                    power_profiles.set_active_profile(profile).await;
                }
            ));
        }
    }
}

glib::wrapper! {
    pub struct PowerProfileSelector(ObjectSubclass<imp::PowerProfileSelector>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PowerProfileSelector {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
