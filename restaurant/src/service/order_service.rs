extern crate chrono;

use chrono::Local;

use crate::order::Order;
use crate::{order_redis, table_redis};
use crate::table::Table;
use crate::table_service::TableService;

use self::chrono::{DateTime, Duration};

/// This is service module that use to handle order's related logic.
/// All the functions are design as functional programming style
/// except order creation, because the order creation would generated UUID as key.
pub struct OrderService {}

impl OrderService {
    /*
     * Below closures are defined for different type of filtering
     * And apply to filter_orders function.
     */

    /// [Closure] used for find out all orders with specific item id
    ///
    /// Examples:
    /// let filter = var![OrderService::item_id_filter(String::from"item1")];
    /// let orders = filter_orders(table, filter);
    pub fn item_id_filter(item_id: String) -> Box<dyn Fn(Order) -> bool> {
        Box::new(move |x: Order| x.item_id.eq(item_id.as_str()))
    }

    /// [Closure] used for filter out expired orders.
    ///
    /// Examples:
    /// let filter = var![OrderService::expired_filter(Local::now())];
    /// let orders = filter_orders(table, filter);
    pub fn not_expired_filter(expire_time: DateTime<Local>) -> Box<dyn Fn(Order) -> bool> {
        Box::new(move |order| {
            let expired_time = order.create_at + Duration::minutes(order.cook_time as i64);
            return expired_time > expire_time;
        })
    }

    /// This function is use to apply and exercise all the filters
    /// and get back the result
    ///
    /// Examples:
    /// let mut filters = Vec::new();
    /// filters.push(OrderService::order_id_filter(String::from"item1"));
    /// filters.push(OrderService::expired_filter(Local::now()));
    /// let orders = filter_orders(table, filters);
    pub fn filter_orders(orders: Vec<Order>, filters: Vec<Box<dyn Fn(Order) -> bool>>) -> Vec<Order> {
        let mut result = Vec::new();
        for order in orders {
            let all_passed = filters.iter()
                .filter(|filter| !filter(order.clone()))
                .count() <= 0;
            if all_passed {
                result.push(order.clone());
            }
        }
        return result;
    }

    /*
     * Other functions for perform order logics
     */

    /// Creating a new orders under the specific table and return all orders under the table.
    /// if the table got 5 orders and adding 7 new orders, it will return 12 orders.
    ///
    /// Examples:
    /// let orders = create_order(table, vec![String::from('new_item_id')], 5);
    pub fn create_order(table_id: String, items: Vec<String>, cook_time: usize) -> Vec<Order> {
        let mut orders = Vec::new();
        items.iter().for_each(|item| {
            orders.push(Order::new(table_id.clone(), item.to_string(), cook_time));
        });
        return orders;
    }

    /// Checking the order can be create or not
    ///
    /// Examples:
    /// let checking = is_able_create_order(table, vec![String::from('new_item_id')]);
    pub fn is_able_create_order(table: Table, orders: Vec<String>) -> bool {
        return !TableService::is_too_much_order(table.clone(), orders.len());
    }

    /*
     * function for handle redis/database
     */

    /// Save the order in both table list and order map in redis
    ///
    /// Examples:
    /// save_order(orders);
    pub fn save_order(orders: Vec<Order>) {
        for order in orders.clone() {
            table_redis::add_order(order.table_id, order.order_id);
        }
        order_redis::add_orders(orders);
    }

    /// Remove order from both table list and order map in redis
    ///
    /// Examples:
    /// let orders = delete_order(order);
    pub fn delete_order(order: Order) {
        table_redis::remove_order(order.table_id.clone(), order.order_id.clone());
        order_redis::remove_order(order.order_id.clone());
    }
}
