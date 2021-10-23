extern crate chrono;

use uuid::Uuid;
use chrono::{Local, DateTime};
use serde::{Deserialize, Serialize};
use crate::datetime_util;

/// define order struct and relate member variable.
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_id: String,
    pub table_id: String,
    pub item_id: String,
    /// 5-15 minutes
    pub cook_time: usize,
    #[serde(with = "datetime_util::json_date_format")]
    pub create_at: DateTime<Local>,
}

impl Order {
    /// Order constructor
    /// Create Order Object by table_id, item_id and cook_time
    ///
    /// Example:
    /// let order = Order::new(String::from('table1'), String::from('item1'), 5);
    pub fn new(table_id: String, item_id: String, cook_time: usize) -> Order {
        let order_id = Uuid::new_v4().to_string();
        Order {
            table_id,
            order_id,
            item_id,
            cook_time,
            create_at: Local::now(),
        }
    }
}
