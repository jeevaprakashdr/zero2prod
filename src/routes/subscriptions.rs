use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    email: String,
    name: String,
}

pub async fn subscribe(subscribe_request: web::Form<SubscribeRequest>) -> impl Responder {
    HttpResponse::Ok().finish()
}
