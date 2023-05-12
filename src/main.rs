use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{ layer::SubscriberExt, EnvFilter, Registry};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));    
    let format_layer = BunyanFormattingLayer::new("zeroToProd".into(), std::io::stdout);
    let subscriber = Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(format_layer);

    set_global_default(subscriber).expect("Failed to subscribe");

    run(listener, connection_pool)?.await
}
