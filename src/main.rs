use actix_web::{App, HttpServer, web, Responder, HttpResponse};
use crate::controllers::cache_controller::CacheController;
use crate::common::utils::json_error_handler::{json_error_handler};

mod common;
mod controllers;
mod services;
mod dto;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health))
            .route("/{key}", web::get().to(CacheController::get))
            .route("/", web::put().to(CacheController::set))
            .route("/{key}", web::delete().to(CacheController::delete))
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
