pub mod config;
mod controller;
pub mod models;
mod openapi;
mod repository;
mod scheduler;
pub mod schema;
pub mod service;
mod structs;

pub use crate::config::Config;
pub use crate::controller::{get_link_by_id, save_link};
use crate::scheduler::start_scheduler;
use crate::service::Service;
use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use std::time::Duration;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::default();
    println!("Api run on {}:{}", config.api_host, config.api_port);
    println!("database_url {}", config.database_url);

    Service::default();

    actix_rt::spawn(async move {
        start_scheduler().await;
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://192.168.0.19:4173")
            .allowed_origin("http://localhost:4173")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(get_link_by_id)
            .service(save_link)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-doc/openapi.json",
                openapi::api_doc::open_api().clone(),
            ))
    })
    .keep_alive(Duration::from_secs(30))
    .bind(format!("{}:{}", config.api_host, config.api_port))?
    .run()
    .await
}
