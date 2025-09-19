use async_once_cell::OnceCell;
use futures::StreamExt;
use gtk::gio;

use dbus::{DBusMenuProxy, StatusNotifierItemProxy, StatusNotifierWatcherProxy};
pub use dbus::{Layout, LayoutProps};

mod dbus;
mod menumodel;

static SESSION: OnceCell<zbus::Connection> = OnceCell::new();
pub async fn get_session() -> zbus::Connection {
    SESSION
        .get_or_init(async {
            zbus::Connection::session()
                .await
                .expect("Could not connect to d-bus")
        })
        .await
        .clone()
}

pub async fn start_server() -> anyhow::Result<()> {
    dbus::StatusNotifierWatcher::start_server().await
}

#[derive(Clone)]
pub struct TrayServer {
    proxy: StatusNotifierWatcherProxy<'static>,
}

impl TrayServer {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = get_session().await;
        let proxy = StatusNotifierWatcherProxy::builder(&conn)
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;

        Ok(TrayServer { proxy })
    }

    pub async fn items(&self) -> anyhow::Result<Vec<TrayItem>> {
        let mut res = Vec::new();
        for name in self.proxy.registered_status_notifier_items().await? {
            if let Ok(item) = TrayItem::new(name).await {
                res.push(item);
            }
        }

        Ok(res)
    }

    pub async fn listen_items_updated(&self) -> anyhow::Result<impl futures::Stream<Item = ()>> {
        let reg = self
            .proxy
            .receive_status_notifier_item_registered()
            .await?
            .map(|_| ());

        let unreg = self
            .proxy
            .receive_status_notifier_item_unregistered()
            .await?
            .map(|_| ());

        Ok(futures::stream::select(reg, unreg))
    }
}

#[derive(Clone)]
pub struct TrayItem {
    notifier_item: StatusNotifierItemProxy<'static>,
    dbus_menu: DBusMenuProxy<'static>,
}

impl TrayItem {
    async fn new(name: String) -> anyhow::Result<Self> {
        let (dest, path) = if let Some(idx) = name.find('/') {
            (name[..idx].to_string(), name[idx..].to_string())
        } else {
            (name, "/StatusNotifierItem".to_string())
        };

        let conn = get_session().await;
        let notifier_item = StatusNotifierItemProxy::builder(&conn)
            .destination(dest.clone())?
            .path(path)?
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;

        let menu_path = notifier_item.menu().await?;
        let dbus_menu = DBusMenuProxy::builder(&conn)
            .destination(dest)?
            .path(menu_path)?
            .build()
            .await?;

        Ok(TrayItem {
            notifier_item,
            dbus_menu,
        })
    }

    pub async fn gicon(&self) -> anyhow::Result<gio::Icon> {
        Ok(gio::ThemedIcon::new(self.notifier_item.icon_name().await?.as_ref()).into())
    }

    pub async fn listen_gicon_updated(&self) -> anyhow::Result<impl futures::Stream<Item = ()>> {
        Ok(self.notifier_item.receive_new_icon().await?.map(|_| ()))
    }

    pub async fn layout(&self) -> anyhow::Result<Layout> {
        Ok(self.dbus_menu.get_layout(0, -1, &[]).await?.1)
    }

    pub async fn listen_layout_updated(&self) -> anyhow::Result<impl futures::Stream<Item = ()>> {
        Ok(self.dbus_menu.receive_layout_updated().await?.map(|_| ()))
    }
}
