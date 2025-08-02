use actix_web::{delete, get, post, Responder};
use crate::{inputs::CreateOrderInput, outputs::CreateOrderResponse};







#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello World!"
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    "Hello, world!"
}

#[post("/order")]
pub async fn create_order() -> impl Responder {
    "Hello, world!"
}

#[delete("/order")]
pub async fn delete_order() -> impl Responder {
    "Hello, world!"
}
