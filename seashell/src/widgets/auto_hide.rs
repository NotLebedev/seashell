mod imp {
    use std::cell::{Cell, RefCell};

    use glib::clone;
    use gtk::{prelude::*, subclass::prelude::*};

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::AutoHide)]
    #[template(string = r"
    using Gtk 4.0;

    template $AutoHide: Box {
        orientation: vertical;

        Revealer revealer {
            child: bind template.child;
            reveal-child: true;
        }
    }
    ")]
    pub struct AutoHide {
        #[template_child]
        pub revealer: gtk::TemplateChild<gtk::Revealer>,
        #[property(get, set)]
        pub child: RefCell<Option<gtk::Widget>>,

        initial_reveal: Cell<bool>,
        hovered: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AutoHide {
        const NAME: &'static str = "AutoHide";
        type Type = super::AutoHide;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AutoHide {
        fn constructed(&self) {
            self.obj().set_size_request(-1, 4);
            let motion_controller = gtk::EventControllerMotion::new();

            let (send, recv) = async_channel::unbounded::<bool>();
            motion_controller.connect(
                "enter",
                false,
                clone!(
                    #[strong]
                    send,
                    move |_| {
                        let _ = send.try_send(true);
                        None
                    }
                ),
            );

            motion_controller.connect(
                "leave",
                false,
                clone!(
                    #[strong]
                    send,
                    move |_| {
                        let _ = send.try_send(false);
                        None
                    }
                ),
            );

            self.obj().add_controller(motion_controller);

            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    while let Ok(hovered) = recv.recv().await {
                        this.hovered(hovered);
                    }
                }
            ));

            // Show for a moment when bar just started
            self.initial_reveal.set(true);
            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    glib::timeout_future_seconds(3).await;
                    this.initital_timeout_ended();
                }
            ));
        }
    }

    impl WidgetImpl for AutoHide {}

    impl BoxImpl for AutoHide {}

    impl AutoHide {
        fn hovered(&self, hovered: bool) {
            self.initial_reveal.set(hovered);
            self.update_revealed();
        }

        fn initital_timeout_ended(&self) {
            self.initial_reveal.set(false);
            self.update_revealed();
        }

        fn update_revealed(&self) {
            self.revealer
                .set_reveal_child(self.hovered.get() || self.initial_reveal.get());
        }
    }
}

glib::wrapper! {
    pub struct AutoHide(ObjectSubclass<imp::AutoHide>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AutoHide {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
