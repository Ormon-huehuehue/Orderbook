use actix_web::{delete, get, http::header::HttpDate, post, web::{Data, Json}, HttpResponse, Responder};
use crate::{inputs::{CreateOrderInput, DeleteOrderInput, Side}, outputs::CreateOrderResponse, orderbook::Orderbook};
use std::sync::{Arc, Mutex};


#[get("/depth")]
pub async fn get_depth(orderbook : Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    let orderbook = orderbook.lock().unwrap();
    let depth = orderbook.get_depth();

    HttpResponse::Ok().json(depth)
}

#[post("/order")]
pub async fn create_order() -> impl Responder {
    
}

#[delete("/order")]
pub async fn delete_order() -> impl Responder {
    "Hello, world!"
}
