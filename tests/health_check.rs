use std::{net::TcpListener};

use actix_web::http::uri::Port;

#[tokio::test]
async fn should_return_ok_response () {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}