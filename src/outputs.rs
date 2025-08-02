use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderResponse {
    pub order_id: String
}


#[derive(Serialize, Deserialize)]
pub struct DeleteOrderResponse {
    pub filled_qty: u32,
    pub average_price: u32
}

