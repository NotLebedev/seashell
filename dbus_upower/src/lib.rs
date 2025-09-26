use anyhow::anyhow;
use futures::{StreamExt, stream};
use zbus::zvariant::OwnedObjectPath;

pub use crate::dbus::DeviceType;
use crate::dbus::{DeviceProxy, PowerProfilesProxy, UPowerProxy};

mod dbus;

pub struct UPower {
    proxy: UPowerProxy<'static>,
}

impl UPower {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = dbus_connections::get_system().await;
        let proxy = UPowerProxy::builder(&conn)
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;

        Ok(UPower { proxy })
    }

    pub async fn get_display_device(&self) -> anyhow::Result<Device> {
        let path = self.proxy.get_display_device().await?;

        Device::new(path).await
    }
}

#[derive(Clone)]
pub struct Device {
    proxy: DeviceProxy<'static>,
}

impl Device {
    async fn new(object: OwnedObjectPath) -> anyhow::Result<Self> {
        let conn = dbus_connections::get_system().await;
        let proxy = DeviceProxy::builder(&conn).path(object)?.build().await?;

        Ok(Device { proxy })
    }

    pub async fn device_type(&self) -> anyhow::Result<DeviceType> {
        self.proxy.type_().await.map_err(anyhow::Error::from)
    }

    pub async fn is_present(&self) -> anyhow::Result<bool> {
        self.proxy.is_present().await.map_err(anyhow::Error::from)
    }

    pub async fn listen_percentage(&self) -> impl futures::Stream<Item = anyhow::Result<f64>> {
        let current =
            stream::once(async { self.proxy.percentage().await.map_err(anyhow::Error::from) });
        let rest = self
            .proxy
            .receive_percentage_changed()
            .await
            .then(async |p| p.get().await.map_err(anyhow::Error::from));

        current.chain(rest)
    }

    pub async fn icon_name(&self) -> impl futures::Stream<Item = anyhow::Result<String>> {
        let current =
            stream::once(async { self.proxy.icon_name().await.map_err(anyhow::Error::from) });
        let rest = self
            .proxy
            .receive_icon_name_changed()
            .await
            .then(async |p| p.get().await.map_err(anyhow::Error::from));

        current.chain(rest)
    }
}

#[derive(Clone, Debug)]
pub struct PowerProfiles {
    proxy: PowerProfilesProxy<'static>,
}

impl PowerProfiles {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = dbus_connections::get_system().await;
        let proxy = PowerProfilesProxy::builder(&conn).build().await?;

        Ok(PowerProfiles { proxy })
    }

    pub async fn set_active_profile(&self, name: PowerProfile) {
        let res = self.proxy.set_active_profile(name.into()).await;
        log::info!("{res:?}");
    }

    pub async fn listen_active_profile(
        &self,
    ) -> impl futures::Stream<Item = anyhow::Result<PowerProfile>> {
        let current = stream::once(async {
            self.proxy
                .active_profile()
                .await
                .map_err(anyhow::Error::from)
                .and_then(PowerProfile::try_from)
        });
        let rest = self
            .proxy
            .receive_active_profile_changed()
            .await
            .then(async |p| {
                p.get()
                    .await
                    .map_err(anyhow::Error::from)
                    .and_then(PowerProfile::try_from)
            });

        current.chain(rest)
    }
}

#[derive(Debug)]
pub enum PowerProfile {
    PowerSaver,
    Balanced,
    Performance,
}

impl From<PowerProfile> for &'static str {
    fn from(val: PowerProfile) -> Self {
        match val {
            PowerProfile::PowerSaver => "power-saver",
            PowerProfile::Balanced => "balanced",
            PowerProfile::Performance => "performance",
        }
    }
}

impl TryFrom<String> for PowerProfile {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "power-saver" => PowerProfile::PowerSaver,
            "balanced" => PowerProfile::Balanced,
            "performance" => PowerProfile::Performance,
            _ => return Err(anyhow!("Unknown power profile {value}")),
        })
    }
}
