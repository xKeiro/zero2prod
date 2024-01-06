use axum::{body::Body, http::Request};

use crate::helpers::create_test_client;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let test_client = create_test_client().await;
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = test_client
        .client
        .request(
            Request::builder()
                .method("POST")
                .uri(&format!("http://{}/subscriptions", test_client.addr))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(body)) // Convert body to axum::body::Body
                .expect("Failed to execute request."),
        )
        .await
        .expect("Failed to get response");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    // Arrange
    let test_client = create_test_client().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = test_client
            .client
            .request(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .uri(&format!("http://{}/subscriptions", test_client.addr))
                    .body(Body::from(invalid_body))
                    .expect("Failed to build request"),
            )
            .await
            .expect("Failed to get response");
        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customized error message on test failure
            "The API did not fail with 422 Unprocessable Content when the payload was {}.",
            error_message
        );
    }
}
