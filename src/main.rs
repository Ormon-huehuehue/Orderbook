use actix_web::{ App, HttpServer, Responder, web::{self, Data}};
use std::sync::{Arc, Mutex};
use crate::orderbook::Orderbook;


mod handlers;
pub mod outputs;
pub mod orderbook;
pub mod inputs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let orderbook = Arc::new(Mutex::new(Orderbook::default()));

    HttpServer::new( move || {
        App::new()
            .app_data(Data::new(orderbook.clone()))
            .service(handlers::get_depth)
            .service(handlers::create_order)
            .service(handlers::delete_order)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}