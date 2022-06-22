#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

mod api;
mod db;
mod errors;
mod models;
mod tests;
mod schema;

use crate::db as database;
use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::{error, http, App, HttpServer, web, HttpResponse};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use log::info;
use serde_json::json;
use std::env;
use std::format;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let json_cfg = web::JsonConfig::default()
        .error_handler(|err, _| {
            error::InternalError::from_response(
                "",
                HttpResponse::BadRequest()
                    .content_type(ContentType::json())
                    .json(
                        json!({
                            "error": err.to_string()
                        })
                    )
            ).into()
        });

    let db_pool = database::get_db_pool(&"DATABASE_URL".to_string());

    let server = HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
            )
            .app_data(json_cfg.clone())
            .app_data(web::Data::new(db_pool.clone()))
            .configure(api::users_controller::register_routes)
    ).bind(format!("{}:{}", 
        env::var("HOST").unwrap_or("127.0.0.1".to_string()), 
        env::var("PORT").unwrap_or("8080".to_string())
    ))?;


    info!("Starting server");
    server.run().await
}