use rand::Rng;
use uuid::Uuid;
extern crate chrono;
use chrono::{Local, DateTime};

#[derive(Clone)]
pub struct Order {
    pub order_id: String,
    pub table_id: String,
    pub item_id: String,
    pub cook_time: usize, // 5-15 minutes
    pub create_at: DateTime<Local>,
}

impl Order {
    pub fn new(table_id:String, item_id:String) -> Order {
        let order_id = Uuid::new_v4().to_string();
        let cook_time:usize = rand::thread_rng().gen_range(5..15);
        Order {
            table_id,
            order_id,
            item_id,
            cook_time,
            create_at: Local::now(),
        }
    }

}
