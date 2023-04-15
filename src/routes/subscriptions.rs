use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
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
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscribe_request.email,
        subscribe_request.name,
        Utc::now()
    )
    .execute(connection_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
