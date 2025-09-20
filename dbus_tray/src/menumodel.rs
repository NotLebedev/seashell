use gio::prelude::*;
use glib::clone;

use super::{Layout, TrayItem};

impl Layout {
    pub fn as_menu_model(&self) -> gio::MenuModel {
        let menu = gio::Menu::new();

        self.2
            .split(|it| it.1.type_.as_ref().is_some_and(|s| s == "separator"))
            .map(Self::build_section)
            .for_each(|section| menu.append_item(&section));

        menu.into()
    }

    fn build_section(group: &[Layout]) -> gio::MenuItem {
        let group_menu = gio::Menu::new();

        group
            .iter()
            .filter_map(Layout::as_menu_item)
            .for_each(|it| group_menu.append_item(&it));

        gio::MenuItem::new_section(None, &group_menu)
    }

    fn as_menu_item(&self) -> Option<gio::MenuItem> {
        if let Some(false) = self.1.visible {
            return None;
        }

        Some(
            if let Some(children_display) = &self.1.children_display
                && children_display == "submenu"
            {
                gio::MenuItem::new_submenu(self.label(), &self.as_menu_model())
            } else {
                gio::MenuItem::new(self.label(), Some(&format!("dbusmenu.{}", self.0)))
            },
        )
    }

    fn label(&self) -> Option<&str> {
        self.1.label.as_deref()
    }

    pub fn as_action_group(&self, tray_item: &TrayItem) -> gio::ActionGroup {
        let action_group = gio::SimpleActionGroup::new();

        self.add_to_action_group(&action_group, tray_item);

        action_group.into()
    }

    fn add_to_action_group(&self, action_group: &gio::SimpleActionGroup, tray_item: &TrayItem) {
        if let Some(true) | None = self.1.enabled {
            let action = if let Some(toggle_type) = &self.1.toggle_type
                && (toggle_type == "checkmark" || toggle_type == "radio")
            {
                gio::SimpleAction::new_stateful(
                    &self.0.to_string(),
                    None,
                    &(self.1.toggle_state == Some(1)).into(),
                )
            } else {
                gio::SimpleAction::new(&self.0.to_string(), None)
            };
            action.connect_activate(clone!(
                #[strong(rename_to = id)]
                self.0,
                #[strong]
                tray_item,
                move |_, _| {
                    glib::spawn_future_local(clone!(
                        #[strong]
                        tray_item,
                        async move {
                            let _ = tray_item.fire_clicked_event(id).await;
                        }
                    ));
                }
            ));
            action_group.add_action(&action);
        }

        self.2
            .iter()
            .for_each(|child| child.add_to_action_group(action_group, tray_item));
    }
}
