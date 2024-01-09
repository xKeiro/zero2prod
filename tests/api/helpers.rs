use std::net::SocketAddr;

use sqlx::{Connection, PgConnection};
use zero2prod::{
    app::{get_listener, run},
    router::app_routes, configuration::get_configuration,
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

pub async fn get_db_connection() -> PgConnection{
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.")
}
