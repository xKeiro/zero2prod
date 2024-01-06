use std::net::SocketAddr;

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use zero2prod::{
    app::{get_listener, run},
    router::app_routes,
};

struct TestClient {
    client: Client<HttpConnector, Body>,
    addr: SocketAddr,
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let test_client = create_test_client().await;
    // Act
    let response = test_client
        .client
        .request(
            Request::builder()
                .uri(format!("http://{}/health_check", test_client.addr))
                .header("Host", "localhost")
                .body(Body::empty())
                .expect("Failed to build request"),
        )
        .await
        .expect("Failed to get response");
    // Assert
    assert!(response.status().is_success());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

async fn create_test_client() -> TestClient {
    let listener = get_listener("0.0.0.0:0")
        .await
        .expect("Failed to bind address");
    let local_addr = listener.local_addr().unwrap();
    let app = app_routes();

    tokio::spawn(async move { run(listener, app).await });
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http::<Body>();
    TestClient {
        addr: local_addr,
        client,
    }
}
