use axum::{body::Body, http::Request};
use http_body_util::BodyExt;

use crate::helpers::create_test_client;

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
    let body = response
        .into_body()
        .collect()
        .await
        .expect("Failed to get response body")
        .to_bytes();
    assert!(body.is_empty());
}
