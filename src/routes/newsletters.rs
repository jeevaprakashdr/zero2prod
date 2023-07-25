use actix_web::{web, HttpResponse};
use sqlx::{PgPool};

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

pub struct ConfirmedSubscriber {
    email: String,
}

pub async fn publish_newsletters(
    _body: web::Json<BodyData>,
    _pool: web::Data<PgPool>
    ) -> HttpResponse {

    //let _subscribers = get_confirmed_subscribers(&pool).await?;

    HttpResponse::Ok().finish()
}

async fn get_confirmed_subscribers(pool: &PgPool) -> Result<Vec<ConfirmedSubscriber>, sqlx::Error> {
    let rows = sqlx::query_as!(
        ConfirmedSubscriber,
        r#"SELECT email FROM subscriptions WHERE status='confirmed'"#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
