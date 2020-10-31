use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use crate::common::cache_strategy::lru_cache_strategy::lru_cache::LruCache;
use crate::common::cache_strategy::i_cache_strategy::ICacheStrategy;

mod common;

#[get("/")]
async fn hello() -> impl Responder {
    let cache: LruCache = ICacheStrategy::new();
    return HttpResponse::Ok().body(cache.get("myKey"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
