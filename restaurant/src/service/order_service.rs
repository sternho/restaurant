extern crate chrono;

use chrono::Local;

use crate::order::Order;
use crate::table::Table;

use self::chrono::{DateTime, Duration};

/// max order can be order for a table
static MAX_ORDER_NUMBER: usize = 100;

/// This is service module that use to handle order's related logic.
/// All the functions are design as functional programming style
/// except order creation, because the order creation would generated UUID as key.
pub struct OrderService {}

impl OrderService {
    /* Below closures are defined for do different kind of filtering
     * And apply to filter_orders function.
     */

    /// Closure used for find out all orders with specific item id
    ///
    /// Examples:
    /// let filter = var![OrderService::item_id_filter(String::from"item1")];
    /// let orders = filter_orders(table, filter);
    pub fn item_id_filter(item_id: String) -> Box<dyn Fn(Order) -> bool> {
        Box::new(move |x: Order| x.item_id.eq(item_id.as_str()))
    }

    /// Closure used for find out the order by order id.
    ///
    /// Examples:
    /// let filter = var![OrderService::order_id_filter(String::from"item1")];
    /// let orders = filter_orders(table, filter);
    pub fn order_id_filter(order_id: String) -> Box<dyn Fn(Order) -> bool> {
        Box::new(move |order| order.order_id.eq(order_id.as_str()))
    }

    /// Closure used for filter out expired orders.
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
    pub fn filter_orders(table: Table, filters: Vec<Box<dyn Fn(Order) -> bool>>) -> Vec<Order> {
        let mut result = Vec::new();
        for order in table.orders {
            let mut all_passed = true;
            for filter in &filters {
                if !filter(order.clone()) {
                    all_passed = false;
                }
            }
            if all_passed {
                result.push(order.clone());
            }
        }
        return result;
    }

    /*
     * Other functions for perform order logics
     */

    /// use to check the order limit for table.
    /// return false if the existing orders and new orders is larger than the limitation
    ///
    /// Examples:
    /// let over_limit = is_too_much_order(table, 10);
    pub fn is_too_much_order(table: Table, new_item_cnt: usize) -> bool {
        table.orders.len() + new_item_cnt > MAX_ORDER_NUMBER
    }

    /// creating a new orders under the specific table and return all orders under the table.
    /// if the table got 5 orders and adding 7 new orders, it will return 12 orders.
    ///
    /// Examples:
    /// let orders = create_order(table, vec![String::from('new_item_id')], 5);
    pub fn create_order(table: Table, items: Vec<String>, cook_time: usize) -> Vec<Order> {
        let mut orders = table.orders.clone();
        items.iter().for_each(|item| {
            orders.push(Order::new(table.table_id.clone(), item.to_string(), cook_time));
        });
        return orders;
    }

    /// removing the order by the specific order_id.
    /// if the table got 3 orders, it will return 2 orders after success.
    ///
    /// Examples:
    /// let orders = delete_order(table, String::from('order_id'));
    pub fn delete_order(table: Table, order_id: String) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| !order.order_id.eq(order_id.as_str()))
            .collect::<Vec<Order>>();
        return orders;
    }

    /*
     * format transfer functions.
     * e.g.: json to object
     */

    /// translate the order to json formatted string
    ///
    /// Examples:
    /// let json_str = to_json(order);
    pub fn to_json(order: Order) -> String {
        let mut json = String::from("{\n");
        json.push_str(&*format!("\t\"order_id\":\"{}\",\n", order.order_id));
        json.push_str(&*format!("\t\"table_id\":\"{}\",\n", order.table_id));
        json.push_str(&*format!("\t\"item_id\":\"{}\",\n", order.item_id));
        json.push_str(&*format!("\t\"cook_time\":{},\n", order.cook_time));
        json.push_str(&*format!("\t\"create_at\":{},\n", order.create_at.format("%Y/%m/%d %H:%M:%S")));
        json.push_str(&*String::from("},\n"));
        return json;
    }

    /// translate the orders to json formatted string
    ///
    /// Examples:
    /// let json_str = to_jsons(order);
    pub fn to_jsons(orders: Vec<Order>) -> String {
        let mut json = String::from("[\n");
        for order in orders {
            let tmp = OrderService::to_json(order);
            json.push_str(&*tmp);
        }
        json.push_str(&*String::from("]\n"));
        return json;
    }
}
