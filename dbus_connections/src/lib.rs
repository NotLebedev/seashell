use async_once_cell::OnceCell;

static SESSION: OnceCell<zbus::Connection> = OnceCell::new();

/// # Panics
/// Could not connect to d-bus session
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
