use crate::domain::SubscriberEmail;
use reqwest::Client;

pub struct EmailClient {
    _sender: SubscriberEmail,
    _http_client: Client,
    _base_url: String,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            _sender: sender,
            _http_client: Client::new(),
            _base_url: base_url,
        }
    }

    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _html_content: &str,
        _text_content: &str,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{faker::{lorem::{en::Sentence, en::Paragraph}, internet::en::SafeEmail}, Fake};
    use wiremock::{matchers::any, MockServer, ResponseTemplate, Mock};

    use crate::{domain::SubscriberEmail, email_client::EmailClient};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender);

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ =email_client.send_email(subscriber_email, &subject, &content, &content)
        .await;
    }
}
