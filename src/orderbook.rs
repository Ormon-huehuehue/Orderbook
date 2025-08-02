use std::collections::HashMap;
use crate::{inputs::Side, outputs::Depth};

pub struct Orderbook{
    pub bids : HashMap<u32, Vec<UserOrder>>,
    pub asks : HashMap<u32, Vec<UserOrder>>,
    pub order_id_index : u32
}

pub struct UserOrder{
    pub user_id : u32, 
    pub qty : u32, 
    pub order_id : u32
}

impl Orderbook{
    pub fn create_order(&mut self, price :u32, quantity : u32, user_id : u32, side: Side){
        let order_id = self.order_id_index;
        self.order_id_index = self.order_id_index + 1;

        if side == Side::Buy {
            self.bids.entry(price).or_insert(Vec::new()).push(UserOrder{
                user_id,
                qty: quantity,
                order_id : order_id
            })
        }
    }
}