use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
extern crate chrono;
use chrono::{Local, DateTime};

#[derive(Clone)]
pub(crate) struct Order {
    pub(crate) order_id: String,
    pub(crate) item_id: String,
    pub(crate) cook_time: i32, // 5-15 minutes
    pub(crate) create_at: DateTime<Local>,
}

impl Order {
    pub fn new(item_id:String) -> Order {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let order_id = since_the_epoch.as_micros().to_string();
        let cook_time:i32 = rand::thread_rng().gen_range(5..15);
        Order {
            order_id,
            item_id,
            cook_time,
            create_at: Local::now(),
        }
    }

    pub fn print(order:Order) {
        println!("[order] item:{}, cook_time:{}",
                 order.item_id, order.cook_time,
        );
    }

    pub fn get_json(order:Order) -> String {
        // let datetime: DateTime<Utc> = order.create_at.into().format("%d/%m/%Y");
        // let item = format!("\"create_at\":\"{}\",", order.create_at);
        let mut json = String::from("{\n");
        json.push_str(&*format!("\t\"order_id\":\"{}\",\n", order.order_id));
        json.push_str(&*format!("\t\"item_id\":\"{}\",\n", order.item_id));
        json.push_str(&*format!("\t\"cook_time\":{},\n", order.cook_time));
        json.push_str(&*format!("\t\"create_at\":{},\n", order.create_at.format("%Y/%m/%d %H:%M:%S")));
        json.push_str(&*String::from("},\n"));
        return json;
        // return String::formatted("{\"item_id\":\"{}\",\"cook_time\":\"{}\",\"create_at\":\"{}\"\"},",
        //                     order.item_id, order.cook_time, order.create_at);

        // let mut json;
        // json = "{"+
        //     "\"item_id\":\"" + order.item_id + "\"," +
        //     "\"cook_time\":\"" + order.cook_time + "\"," +
        //     "\"create_at\":\"" + order.create_at + "\"" +
        // "},";
        // return json;
    }
}
