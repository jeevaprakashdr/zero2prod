use reqwest::Client;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::test]
async fn should_return_ok_response() {
    // Arrange
    let address = spawn_app();

    let client = Client::new();

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    let client = Client::new();

    // Act
    let body = "name=rusty&email=ichbeginenrusty%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, "ichbeginenrusty@gmail.com");
    assert_eq!(saved.name, "rusty");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arrange
    let app_url = spawn_app();
    let client = Client::new();
    let test_cases = vec![
        ("name=rusty", "missing the email"),
        ("email=ichbeginenrusty%40gmail.com", "missing the name"),
        ("", "missing email and name"),
    ];

    for (body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", app_url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        //Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
