use actix_web::{web, App, Responder, HttpServer, HttpResponse, dev::Server};

async fn health_check() -> impl Responder{
    HttpResponse::Ok().finish()
} 


pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .bind("localhost:8001")
    ?.run();
    Ok(server)
}
