extern crate redis;

use redis::Commands;

use crate::datetime_util;
use crate::order::Order;

static REDIS_ADDRESS: &str = "redis://127.0.0.1/";
static REDIS_DATETIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

/// Save the order objects to redis hash
///
/// Redis command:
/// hset order_id table_id
/// hset order_id item_id
/// hset order_id cook_time
/// hset order_id create_at
pub fn add_orders(orders: Vec<Order>) -> bool {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();

    let mut all_result = true;
    for order in orders {
        let result = con.hset::<String, &str, String, String>(
            order.order_id.clone(), "table_id", order.table_id);
        all_result = all_result && result.is_ok();
        let result = con.hset::<String, &str, String, String>(
            order.order_id.clone(), "item_id", order.item_id);
        all_result = all_result && result.is_ok();
        let result = con.hset::<String, &str, usize, String>(
            order.order_id.clone(), "cook_time", order.cook_time);
        all_result = all_result && result.is_ok();
        let create_at = order.create_at.format(REDIS_DATETIME_FORMAT);
        let result = con.hset::<String, &str, String, String>(
            order.order_id.clone(), "create_at", create_at.to_string());
        all_result = all_result && result.is_ok();
    }
    return all_result;
}

/// remove order object in redis hash
///
/// Redis command:
/// del order_id
pub fn remove_order(order_id: String) -> bool {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();
    let result = con.del::<String, String>(order_id);
    result.is_ok()
}

/// get orders from redis
///
/// Redis command:
/// hget order_id table_id
/// hget order_id item_id
/// hget order_id cook_time
/// hget order_id create_at
pub fn fetch_orders(order_ids: Vec<String>) -> Vec<Order> {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();

    let mut result = Vec::new();
    for order_id in order_ids {
        let field_size: i32 = con.hlen::<String, i32>(order_id.clone()).unwrap();
        if field_size > 0 {
            let create_at: String = con.hget(order_id.clone(), "create_at").unwrap();
            let create_at = datetime_util::to_date_str(create_at.as_str(), REDIS_DATETIME_FORMAT);
            let order = Order {
                order_id: order_id.clone(),
                table_id: con.hget(order_id.clone(), "table_id").unwrap(),
                item_id: con.hget(order_id.clone(), "item_id").unwrap(),
                cook_time: con.hget(order_id.clone(), "cook_time").unwrap(),
                create_at,
            };
            result.push(order);
        }
    }

    return result;
}

/// get one order from redis
///
/// Redis command:
/// hget order_id table_id
/// hget order_id item_id
/// hget order_id cook_time
/// hget order_id create_at
pub fn fetch(order_id: String) -> Option<Order> {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();
    let field_size: i32 = con.hlen(order_id.clone()).unwrap();
    if field_size > 0 {
        let create_at: String = con.hget::<String, &str, String>(order_id.clone(), "create_at").unwrap();
        let create_at = datetime_util::to_date_str(create_at.as_str(), REDIS_DATETIME_FORMAT);
        let order = Order {
            order_id: order_id.clone(),
            table_id: con.hget::<String, &str, String>(order_id.clone(), "table_id").unwrap(),
            item_id: con.hget::<String, &str, String>(order_id.clone(), "item_id").unwrap(),
            cook_time: con.hget::<String, &str, usize>(order_id.clone(), "cook_time").unwrap(),
            create_at,
        };
        return Option::from(order);
    }
    return Option::None;
}
