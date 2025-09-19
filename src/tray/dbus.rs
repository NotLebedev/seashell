use futures::StreamExt;
use log::{info, warn};
use zbus::{
    Result,
    fdo::{DBusProxy, RequestNameFlags, RequestNameReply},
    interface,
    message::Header,
    names::{BusName, UniqueName, WellKnownName},
    object_server::SignalEmitter,
    proxy,
    zvariant::{self, OwnedObjectPath, OwnedValue, Type},
};

use crate::tray::get_session;

const NAME: WellKnownName =
    WellKnownName::from_static_str_unchecked("org.kde.StatusNotifierWatcher");
const OBJECT_PATH: &str = "/StatusNotifierWatcher";

#[derive(Debug, Default)]
pub struct StatusNotifierWatcher {
    items: Vec<(UniqueName<'static>, String)>,
}

impl StatusNotifierWatcher {
    pub async fn start_server() -> anyhow::Result<()> {
        let connection = get_session().await;
        connection
            .object_server()
            .at(OBJECT_PATH, StatusNotifierWatcher::default())
            .await?;

        let interface = connection
            .object_server()
            .interface::<_, StatusNotifierWatcher>(OBJECT_PATH)
            .await?;

        let dbus_proxy = DBusProxy::new(&connection).await?;
        let mut name_owner_changed_stream = dbus_proxy.receive_name_owner_changed().await?;

        let flags = RequestNameFlags::AllowReplacement.into();
        if dbus_proxy.request_name(NAME, flags).await? == RequestNameReply::InQueue {
            warn!("Bus name '{NAME}' already owned");
        }

        let internal_connection = connection.clone();
        gtk::glib::spawn_future(async move {
            let mut has_bus_name = false;
            let unique_name = internal_connection.unique_name().map(|x| x.as_ref());
            while let Some(evt) = name_owner_changed_stream.next().await {
                let Ok(args) = evt.args() else {
                    continue;
                };

                if args.name.as_ref() == NAME {
                    if args.new_owner.as_ref() == unique_name.as_ref() {
                        info!("Acquired bus name: {NAME}");
                        has_bus_name = true;
                    } else if has_bus_name {
                        info!("Lost bus name: {NAME}");
                        has_bus_name = false;
                    }
                } else if let BusName::Unique(name) = &args.name {
                    let mut interface = interface.get_mut().await;
                    if let Some(idx) = interface
                        .items
                        .iter()
                        .position(|(unique_name, _)| unique_name == name)
                    {
                        let Ok(emitter) = SignalEmitter::new(&internal_connection, OBJECT_PATH)
                        else {
                            continue;
                        };

                        let service = interface.items.remove(idx).1;
                        let _ = StatusNotifierWatcher::status_notifier_item_unregistered(
                            &emitter, &service,
                        )
                        .await;
                    }
                }
            }
        });

        Ok(())
    }
}

#[interface(
    name = "org.kde.StatusNotifierWatcher",
    proxy(
        gen_blocking = false,
        default_service = "org.kde.StatusNotifierWatcher",
        default_path = "/StatusNotifierWatcher",
    )
)]
impl StatusNotifierWatcher {
    async fn register_status_notifier_item(
        &mut self,
        service: &str,
        #[zbus(header)] header: Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) {
        let Some(sender) = header.sender() else {
            warn!("Unknown sender in register_status_notifier_item");
            return;
        };

        let service = if service.starts_with('/') {
            format!("{sender}{service}")
        } else {
            service.to_string()
        };

        let _ = Self::status_notifier_item_registered(&emitter, &service).await;

        self.items.push((sender.to_owned(), service));
    }

    fn register_status_notifier_host(&mut self, _service: &str) {}

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Vec<String> {
        self.items.iter().map(|(_, x)| x.clone()).collect()
    }

    #[zbus(property)]
    fn is_status_notifier_host_registered(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn protocol_version(&self) -> i32 {
        0
    }

    #[zbus(signal)]
    async fn status_notifier_item_registered(
        emitter: &SignalEmitter<'_>,
        service: &str,
    ) -> Result<()>;

    #[zbus(signal)]
    async fn status_notifier_item_unregistered(
        emitter: &SignalEmitter<'_>,
        service: &str,
    ) -> Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_registered(emitter: &SignalEmitter<'_>) -> Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_unregistered(emitter: &SignalEmitter<'_>) -> Result<()>;
}

#[derive(Clone, Debug, zvariant::Value)]
pub struct Icon {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<u8>,
}

#[proxy(interface = "org.kde.StatusNotifierItem")]
pub trait StatusNotifierItem {
    #[zbus(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn icon_pixmap(&self) -> zbus::Result<Vec<Icon>>;

    #[zbus(property)]
    fn menu(&self) -> zbus::Result<OwnedObjectPath>;

    #[zbus(signal)]
    fn new_icon(&self) -> Result<()>;
}

/// Menu items are represented with a unique numeric id and a dictionary of properties [`LayoutProps`]
#[derive(Clone, Debug, Type)]
#[zvariant(signature = "(ia{sv}av)")]
pub struct Layout(pub i32, pub LayoutProps, pub Vec<Layout>);

impl<'a> serde::Deserialize<'a> for Layout {
    fn deserialize<D: serde::Deserializer<'a>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let (id, props, children) =
            <(i32, LayoutProps, Vec<(zvariant::Signature, Self)>)>::deserialize(deserializer)?;
        Ok(Self(id, props, children.into_iter().map(|x| x.1).collect()))
    }
}

/// To reduce the amount of DBus traffic, a property should only be returned if its
/// value is not the default value.
///
/// ## Note:
/// The following values present in com.canonical.dbusmenu spec but not
/// implemented here:
///
/// | Name      | Type                       | Description                                                                                                                                                                                                                                                                                                                                                                                                          | Default Value |
/// |-----------|----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------|
/// | icon-name | string                     | Icon name of the item, following the freedesktop.org icon spec.                                                                                                                                                                                                                                                                                                                                                      | ""            |
/// | icon-data | binary                     | PNG data of the icon.                                                                                                                                                                                                                                                                                                                                                                                                | Empty         |
/// | shortcut  | array of arrays of strings | The shortcut of the item. Each array represents the key press in the list of keypresses. Each list of strings contains a list of modifiers and then the key that is used. The modifier strings allowed are: "Control", "Alt", "Shift" and "Super". - A simple shortcut like Ctrl+S is represented as: [["Control", "S"]] - A complex shortcut like Ctrl+Q, Alt+X is represented as: [["Control", "Q"], ["Alt", "X"]] | Empty         |
#[derive(Clone, Debug, Type, zvariant::DeserializeDict)]
#[zvariant(signature = "dict")]
pub struct LayoutProps {
    /// Can be one of:
    ///
    /// * "standard": an item which can be clicked to trigger an action or
    ///   show another menu
    /// * "separator": a separator
    /// Vendor specific types can be added by prefixing them with "x--".
    ///
    /// **Default**: "standard"
    #[zvariant(rename = "type")]
    pub type_: Option<String>,

    /// Text of the item, except that:
    ///
    /// * two consecutive underscore characters "__" are displayed as a single underscore,
    /// * any remaining underscore characters are not displayed at all,
    /// * the first of those remaining underscore characters (unless it is the last
    ///   character in the string) indicates that the following character is the access key.
    ///
    /// **Default**: ""
    pub label: Option<String>,

    /// Whether the item can be activated or not.
    ///
    /// **Default**: true
    pub enabled: Option<bool>,

    /// True if the item is visible in the menu.
    ///
    /// **Default**: true
    pub visible: Option<bool>,

    /// If the item can be toggled, this property should be set to:
    ///
    /// * "checkmark": Item is an independent togglable item
    /// * "radio": Item is part of a group where only one item can be toggled at a time
    /// * "": Item cannot be toggled
    ///
    /// **Default**: ""
    #[zvariant(rename = "toggle-type")]
    pub toggle_type: Option<String>,

    /// Describe the current state of a "togglable" item. Can be one of:
    ///
    /// * 0 = off
    /// * 1 = on
    /// * anything else = indeterminate
    ///
    /// Note: The implementation does not itself handle ensuring that only one item in a
    /// radio group is set to "on", or that a group does not have "on" and "indeterminate"
    /// items simultaneously; maintaining this policy is up to the toolkit wrappers.
    ///
    /// **Default**: -1
    #[zvariant(rename = "toggle-state")]
    pub toggle_state: Option<i32>,

    /// If the menu item has children this property should be set to "submenu".
    /// **Default**: ""
    #[zvariant(rename = "children-display")]
    pub children_display: Option<String>,
}

#[proxy(interface = "com.canonical.dbusmenu")]
pub trait DBusMenu {
    /// Provides the layout and propertiers that are attached to the entries that are in
    /// the layout. It only gives the items that are children of the item that is specified
    /// in `parentId`. It will return all of the properties or specific ones depending of
    /// the value in `propertyNames`. The format is recursive, where the second 'v' is in
    /// the same format as the original 'a(ia{sv}av)'. Its content depends on the value
    /// of `recursionDepth`
    ///
    /// ## Arguments
    /// * `parent_id`: The ID of the parent node for the layout. For grabbing the layout
    ///   from the root node use zero.
    /// * `recuresion_depth`: The amount of levels of recursion to use. This affects the
    ///   content of the second variant array.
    ///     * -1: deliver all the items under the `parentId`.
    ///     * 0: no recursion, the array will be empty.
    ///     * n: array will contains
    ///   items up to 'n' level depth.
    /// * `property_names`: The list of item properties we are interested in. If there are no entries in the list all of the properties will be sent.
    ///
    /// ## Return values
    /// 0. The revision number of the layout. For matching with layoutUpdated signals.
    /// 1. The layout, as a recursive structure.
    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: &[&str],
    ) -> zbus::Result<(u32, Layout)>;

    /// This is called by the applet to notify the application an event happened on a
    /// menu item. `event_id` can be one of the following:
    /// * "clicked"
    /// * "hovered"
    ///
    /// Vendor specific events can be added by prefixing them with "x-<vendor>-"
    ///
    /// ## Arguments
    /// * `id`: the id of the item which received the event
    /// * `event_id`: the type of event
    /// * `data`: event-specific data
    /// * `timestamp`: The time that the event occured if available or the time the
    ///   message was sent if not
    fn event(&self, id: i32, event_id: &str, data: &OwnedValue, timestamp: u32)
    -> zbus::Result<()>;

    /// Triggered by the application to notify display of a layout update, up to revision
    ///
    /// ## Arguments
    /// * `revision`: The revision of the layout that we're currently on
    /// * `parent`: If the layout update is only of a subtree, this is the parent item
    /// for the entries that have changed. It is zero if the whole layout should
    /// be considered invalid.
    #[zbus(signal)]
    fn layout_updated(&self, revision: u32, parent: i32) -> zbus::Result<()>;
}
