use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(_subscribe_request: web::Form<SubscribeRequest>) -> impl Responder {
    HttpResponse::Ok().finish()
}
