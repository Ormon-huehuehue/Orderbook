use std::collections::{BTreeMap, HashMap};
use crate::{inputs::{CreateOrderInput, DeleteOrderInput, Side}, outputs::{CreateOrderResponse, DeleteOrderResponse}};
use serde::{Serialize, Deserialize};


pub struct Orderbook{
    pub bids : HashMap<String, Vec<OpenOrder>>,
    pub asks : HashMap<String, Vec<OpenOrder>>,
    pub order_id_index : u32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Depth {
    pub price: f64,
    pub quantity: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DepthResponse {
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}

#[derive(Clone)]
pub struct OpenOrder{
    pub user_id : String, 
    pub quantity : u64, 
    pub order_id : String,
    pub price : f64,
    pub filled_quantity : f64,
    pub side : Side
}

impl Orderbook{
    pub fn create_order(&mut self, order : CreateOrderInput){
        let order_id = self.order_id_index.to_string();
        self.order_id_index = self.order_id_index + 1;

        let open_order = OpenOrder{
            user_id : order.user_id,
            quantity : order.quantity,
            order_id : order_id, 
            price : order.price,
            filled_quantity : 0.0,
            side : order.side.clone()
        };

        match &order.side{
            Side::Buy => {
                self.bids.entry(order.price.to_string()).or_insert(Vec::new()).push(open_order)
            }
            Side::Sell => {
                self.asks.entry(order.price.to_string()).or_insert(Vec::new()).push(open_order)
            }
        }
    }


    pub fn delete_order(&mut self, order: DeleteOrderInput){
        //find and remove from bids
        if let Some(price) = self.bids.iter().find_map( |(price,orders)| {
            if orders.iter().any(|o| o.order_id == order.order_id){
                Some(price.clone())
            }
            else{
                None
            }
        }) {
            if let Some(orders) = self.bids.get_mut(&price){
                orders.retain(|o| o.order_id != order.order_id);
            }
        }

        //find and remove from asks
        if let Some(price) = self.asks.iter().find_map( |(price,orders)| {
            if orders.iter().any(|o| o.order_id == order.order_id){
                Some(price.clone())
            }
            else{
                None
            }
        }) {
            if let Some(orders) = self.asks.get_mut(&price){
                orders.retain(|o| o.order_id != order.order_id);
            }
        }

        
    }

    //convert this to a readable struct 
    pub fn get_depth(&self)-> DepthResponse{
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        for(price, orders) in self.bids.iter() {
            let depth = Depth{
                price : price.parse().unwrap(),
                quantity : orders.iter().map(|o| o.quantity).sum()
            };

            bids.push(depth);
        }
       
        for(price, orders) in self.asks.iter() {
            let depth = Depth{
                price : price.parse().unwrap(),
                quantity : orders.iter().map(|o| o.quantity).sum()
            };

            asks.push(depth);
        }

        DepthResponse {bids, asks}
    }
}