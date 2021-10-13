use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use uuid::Uuid;
extern crate chrono;
use chrono::{Local, DateTime};
use self::chrono::Duration;

#[derive(Clone)]
pub(crate) struct Order {
    pub(crate) order_id: String,
    pub(crate) item_id: String,
    pub(crate) cook_time: i32, // 5-15 minutes
    pub(crate) create_at: DateTime<Local>,
}

impl Order {
    pub fn new(item_id:String) -> Order {
        let order_id = Uuid::new_v4().to_string();
        // let cook_time:i32 = rand::thread_rng().gen_range(5..15);
        let cook_time:i32 = 1;
        Order {
            order_id,
            item_id,
            cook_time,
            create_at: Local::now(),
        }
    }

    pub fn is_order_expired(order:Order) -> bool {
        let expired_time = order.create_at + Duration::minutes(order.cook_time as i64);
        return expired_time > Local::now();
    }

    pub fn print(order:Order) {
        println!("[order] item:{}, cook_time:{}",
                 order.item_id, order.cook_time,
        );
    }

    pub fn to_json(order:Order) -> String {
        let mut json = String::from("{\n");
        json.push_str(&*format!("\t\"order_id\":\"{}\",\n", order.order_id));
        json.push_str(&*format!("\t\"item_id\":\"{}\",\n", order.item_id));
        json.push_str(&*format!("\t\"cook_time\":{},\n", order.cook_time));
        json.push_str(&*format!("\t\"create_at\":{},\n", order.create_at.format("%Y/%m/%d %H:%M:%S")));
        json.push_str(&*String::from("},\n"));
        return json;
    }

    pub fn to_jsons(orders:Vec<Order>) -> String {
        let mut json = String::from("[\n");
        for order in orders {
            let tmp = Order::to_json(order);
            json.push_str(&*tmp);
        }
        json.push_str(&*String::from("]\n"));
        return json
    }

}
