#[tokio::test]
async fn should_return_ok_response () {
    // Arrange
    spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
    .get("http://localhost:8001/health_check")
    .send()
    .await
    .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}