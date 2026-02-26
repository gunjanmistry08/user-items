use crate::app::configure_routes;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

mod app;
mod config;
mod domain;
mod producer;
mod routes;
mod state;

use config::Config;
use producer::kafka_producer::KafkaProducer;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let producer = KafkaProducer::new(&config.kafka_brokers);

    let state = AppState {
        producer: Arc::new(producer),
        kafka_topic: config.kafka_topic.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(configure_routes)
    })
    .bind((config.server_host, config.server_port))?
    .run()
    .await
}
