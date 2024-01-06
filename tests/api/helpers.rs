use std::net::SocketAddr;

use axum::body::Body;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use zero2prod::{
    app::{get_listener, run},
    router::app_routes,
};

pub struct TestClient {
    pub client: Client<HttpConnector, Body>,
    pub addr: SocketAddr,
}

pub async fn create_test_client() -> TestClient {
    let listener = get_listener("0.0.0.0:0")
        .await
        .expect("Failed to bind address");
    let local_addr = listener.local_addr().expect("Failed to get local address");
    let app = app_routes();

    tokio::spawn(async move { run(listener, app).await });
    let client = Client::builder(TokioExecutor::new()).build_http::<Body>();
    TestClient {
        addr: local_addr,
        client,
    }
}
