use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    subscribe_request: web::Form<SubscribeRequest>,
    connection_pool: web::Data<PgPool>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber",
    %request_id,
    subscribe_email=%subscribe_request.email,
    subscribe_name=%subscribe_request.name);

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "#,
        request_id,
        subscribe_request.email,
        subscribe_request.name,
        Utc::now()
    )
    .execute(connection_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(
                "request_id {} = Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
