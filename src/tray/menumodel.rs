use gtk::gio;

use super::Layout;

impl Layout {
    pub fn as_menu_model(&self) -> gio::MenuModel {
        let menu = gio::Menu::new();

        menu.into()
    }

    pub fn as_action_group(&self) -> gio::ActionGroup {
        let action_group = gio::SimpleActionGroup::new();

        action_group.into()
    }
}
