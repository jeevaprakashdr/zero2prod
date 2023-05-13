use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscribe_request, connection_pool),
    fields(
    request_id = %Uuid::new_v4(),
    subscriber_email = %subscribe_request.email,
    subscriber_name = %subscribe_request.name
    )
    )]
pub async fn subscribe(
    subscribe_request: web::Form<SubscribeRequest>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&subscribe_request, &connection_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscribe_request, connection_pool)
)]
pub async fn insert_subscriber(
    subscribe_request: &SubscribeRequest,
    connection_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscribe_request.email,
        subscribe_request.name,
        Utc::now()
    )
    .execute(connection_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
