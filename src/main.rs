use sqlx::PgPool;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber();
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?; 
    
    run(listener, connection_pool)?.await
}

