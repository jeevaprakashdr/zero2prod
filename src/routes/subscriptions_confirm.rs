use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters))]
pub async fn confirm(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> HttpResponse {
    let subscriber_id = match get_subscriber_id(&pool, &parameters).await {
        Ok(subscriber_id_option) => subscriber_id_option,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match subscriber_id {
        None => HttpResponse::Unauthorized().finish(),
        Some(id) => {
            if confirm_subscription(&pool, id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::Ok().finish()
        }
    }
}

#[tracing::instrument(name = "update subscription status to 'confirm'", skip(pool, id))]
pub async fn confirm_subscription(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"Update subscriptions set status = 'confirmed' where id =$1"#,
        id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update subscription status: {}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "get subscriber id", skip(pool, parameters))]
pub async fn get_subscriber_id(
    pool: &PgPool,
    parameters: &Parameters,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens WHERE subscription_token=$1",
        parameters.subscription_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(result.map(|r| r.subscriber_id))
}
