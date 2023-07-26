use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app: crate::helpers::TestApp = spawn_app().await;
    let body = "name=rusty&email=ichbeginenrusty%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let response = app.post_subscription(body.into()).await;

    //Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    // Arrange
    let app: crate::helpers::TestApp = spawn_app().await;
    let body = "name=rusty&email=ichbeginenrusty%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let _ = app.post_subscription(body.into()).await;

    //Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ichbeginenrusty@gmail.com");
    assert_eq!(saved.name, "rusty");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=rusty", "missing the email"),
        ("email=ichbeginenrusty%40gmail.com", "missing the name"),
        ("", "missing email and name"),
    ];

    for (body, error_message) in test_cases {
        // Act
        let response = app.post_subscription(body.into()).await;

        //Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    // Act
    for (body, description) in test_cases {
        // Act
        let response = app.post_subscription(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn subscriber_sends_confirmation_email_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=rusty&email=ichbeginenrusty%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscription(body.into()).await;

    //Assert
    // as part of the mock expectations above
}

#[tokio::test]
async fn subscriber_sends_confirmation_email_with_a_link() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=rusty&email=ichbeginenrusty%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscription(body.into()).await;

    //Assert
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn subscriber_fails_if_there_is_database_error() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=rusty&email=ichbeginenrusty%40gmail.com";

    sqlx::query!("ALTER TABLE subscription_tokens DROP COLUMN subscription_token;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    // Act
    let response = app.post_subscription(body.into()).await;

    // Assert
    assert_eq!(response.status().as_u16(), 500);
}
