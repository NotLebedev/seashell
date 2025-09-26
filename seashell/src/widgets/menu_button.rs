mod imp {
    use std::cell::RefCell;

    use glib::clone;
    use gtk::{prelude::*, subclass::prelude::*};

    use crate::widgets::PowerProfileSelector;

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::MenuButton)]
    #[template(string = r#"
    using Gtk 4.0;

    template $MenuButton: Box {
        styles ["menuButton"]
        MenuButton button {
            child: Box inner {};

            popover: Popover {
                has-arrow: false;

                child: Box {
                    orientation: vertical;

                    Calendar {}
                    $PowerProfileSelector {}
                };
            };
        }
    }
    "#)]
    pub struct MenuButton {
        #[template_child]
        pub button: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub inner: TemplateChild<gtk::Box>,
        #[property(get, set, nullable, name = "menu-anchor")]
        pub menu_anchor: RefCell<Option<gtk::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MenuButton {
        const NAME: &'static str = "MenuButton";
        type Type = super::MenuButton;
        type ParentType = gtk::Box;
        type Interfaces = (gtk::Buildable,);

        fn class_init(klass: &mut Self::Class) {
            PowerProfileSelector::ensure_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for MenuButton {
        fn constructed(&self) {
            if let Some(popover) = self.button.popover() {
                popover.connect_show(clone!(
                    #[weak(rename_to = this)]
                    self,
                    move |popover| this.point_to_anchor(popover)
                ));
            }
        }
    }

    impl WidgetImpl for MenuButton {}

    impl BoxImpl for MenuButton {}

    impl BuildableImpl for MenuButton {
        #[allow(
            clippy::expect_used,
            reason = "App can not run with incorrect children in layout"
        )]
        fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
            if self.button.is_bound() {
                self.inner.append(
                    child
                        .downcast_ref::<gtk::Widget>()
                        .expect("Child expected to be a widget"),
                );
            } else {
                self.parent_add_child(builder, child, type_);
            }
        }
    }

    impl MenuButton {
        /// Make popover appear under the menu-anchor widget
        ///
        /// The trick here is to calculate position of anchor
        /// relative to button widget using [`WidgetExt::compute_bounds`]
        /// and then use [`PopoverExt::set_pointing_to`] which uses
        /// coordinates of parent (button)
        fn point_to_anchor(&self, popover: &gtk::Popover) {
            if let Some(anchor) = &*self.menu_anchor.borrow()
                && let Some(bounds) = anchor.compute_bounds(&*self.obj())
            {
                popover.set_pointing_to(Some(&gdk::Rectangle::new(
                    bounds.x() as i32,
                    bounds.y() as i32,
                    bounds.width() as i32,
                    bounds.height() as i32,
                )));
            }
        }
    }
}

glib::wrapper! {
    pub struct MenuButton(ObjectSubclass<imp::MenuButton>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl MenuButton {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
