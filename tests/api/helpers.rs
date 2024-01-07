use std::net::SocketAddr;

use zero2prod::{
    app::{get_listener, run},
    router::app_routes,
};

pub async fn spawn_app() -> SocketAddr {
    let listener = get_listener("0.0.0.0:0")
        .await
        .expect("Failed to bind address");
    let local_addr = listener.local_addr().expect("Failed to get local address");
    let app = app_routes();

    tokio::spawn(async move { run(listener, app).await });
    local_addr
}
