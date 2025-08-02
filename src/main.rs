use actix_web::{ App, HttpServer};
use std::sync::{Arc, Mutex};



mod handlers;
pub mod inputs;
pub mod outputs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .service(handlers::get_depth)
            .service(handlers::create_order)
            .service(handlers::delete_order)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}