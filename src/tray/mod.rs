use async_once_cell::OnceCell;
use futures::StreamExt;
use gtk::gio;

use crate::tray::dbus::{StatusNotifierItemProxy, StatusNotifierWatcherProxy};

mod dbus;

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
    dest: String,
    proxy: StatusNotifierItemProxy<'static>,
}

impl TrayItem {
    async fn new(name: String) -> anyhow::Result<Self> {
        let (dest, path) = if let Some(idx) = name.find('/') {
            (name[..idx].to_string(), name[idx..].to_string())
        } else {
            (name, "/StatusNotifierItem".to_string())
        };

        let proxy = StatusNotifierItemProxy::builder(&get_session().await)
            .destination(dest.clone())?
            .path(path)?
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;
        Ok(TrayItem { dest, proxy })
    }

    pub async fn gicon(&self) -> anyhow::Result<gio::Icon> {
        Ok(gio::ThemedIcon::new(self.proxy.icon_name().await?.as_ref()).into())
    }

    pub async fn listen_gicon_updated(&self) -> anyhow::Result<impl futures::Stream<Item = ()>> {
        Ok(self.proxy.receive_new_icon().await?.map(|_| ()))
    }
}
