extern crate redis;

use crate::table::Table;

use redis::Commands;

static REDIS_ADDRESS: &str = "redis://127.0.0.1/";

/// Save the table data to redis list
///
/// redis command
/// lpush table_id order_id
pub fn add_order(table_id: String, order_id: String) -> bool {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();
    let result = con.lpush::<String, String, i32>(table_id, order_id);
    result.is_ok()
}

/// delete the table data in redis list
///
/// redis command
/// lpush table_id order_id
pub fn remove_order(table_id: String, order_id: String) -> i32 {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();
    let result: i32 = con.lrem::<String, String, i32>(table_id, -1, order_id).unwrap();
    result
}

/// get table data from redis list
///
/// redis command
/// lrange table_id 0 -1
pub fn fetch(table_id: String) -> Table {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();
    let result = con.lrange(table_id.clone(), 0, -1);
    let mut table = Table::new(table_id.clone());
    if result.is_ok() {
        table.orders = result.unwrap();
    }
    table
}
