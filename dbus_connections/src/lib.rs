use async_once_cell::OnceCell;

static SYSTEM: OnceCell<zbus::Connection> = OnceCell::new();
static SESSION: OnceCell<zbus::Connection> = OnceCell::new();

/// # Panics
/// Could not connect to d-bus session bus
pub async fn get_session() -> zbus::Connection {
    #[allow(
        clippy::expect_used,
        reason = "App can not run properly withou d-bus connection"
    )]
    SESSION
        .get_or_init(async {
            zbus::Connection::session()
                .await
                .expect("Could not connect to d-bus")
        })
        .await
        .clone()
}

/// # Panics
/// Could not connect to d-bus system bus
pub async fn get_system() -> zbus::Connection {
    #[allow(
        clippy::expect_used,
        reason = "App can not run properly withou d-bus connection"
    )]
    SYSTEM
        .get_or_init(async {
            zbus::Connection::system()
                .await
                .expect("Could not connect to d-bus")
        })
        .await
        .clone()
}
