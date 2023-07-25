use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::{spawn_app, TestApp, self};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_subscribers(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_json_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        }
    });

    // Act
    let response = app.post_newsletters(newsletter_json_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn newsletters_are_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscribers(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_json_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as HTML</p>",
        }
    });

    // Act
    let response = app.post_newsletters(newsletter_json_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (serde_json::json!({
            "title": "Newsletter title",
        }), "missing content"),
        (serde_json::json!({
            "content": {
                "text": "Newsletter body as plain text",
                "html": "<p>Newsletter body as HTML</p>",
            }
        }), "missing title")
    ];

    // Act
    for (invalid_body, error_message) in test_cases
    {
        let response = app.post_newsletters(invalid_body).await;

        // Assert
        assert_eq!(400,
            response.status().as_u16(),
            "API did not fail with 400 Bad Request when the payload was {}",
            error_message);
    }
}

async fn create_confirmed_subscribers(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscribers(app).await;

    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

async fn create_unconfirmed_subscribers(app: &TestApp) -> crate::helpers::ConfirmationLinks {
    let body = "name=rusty&email=ichbeginenrusty%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscription(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_requests = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(&email_requests)
}
