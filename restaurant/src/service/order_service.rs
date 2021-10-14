use std::sync::Mutex;
use chrono::{Local, DateTime};
use self::chrono::Duration;

extern crate chrono;
use crate::order::Order;
use crate::table::Table;

static MAX_ORDER_NUMBER: usize = 100;

pub struct OrderService {

}

impl OrderService {

    pub fn is_order_expired(order:Order) -> bool {
        let expired_time = order.create_at + Duration::minutes(order.cook_time as i64);
        return expired_time > Local::now();
    }
    pub fn is_too_much_order(table:Table, new_item_cnt:usize) -> bool {
        table.orders.len()+new_item_cnt > MAX_ORDER_NUMBER
    }

    pub fn create_order(table:Table, items:Vec<String>) -> Vec<Order> {
        let mut orders = table.orders.clone();
        items.iter().for_each(|item|{
            orders.push(Order::new(table.table_id.clone(), item.to_string()));
        });
        return orders;
    }

    pub fn delete_order(table:Table, order_id:String) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| !order.order_id.eq(order_id.as_str()))
            .collect::<Vec<Order>>();
        return orders;
    }

    pub fn get_orders_active(table:Table) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| OrderService::is_order_expired(order.clone()))
            .collect::<Vec<Order>>();
        return orders;
    }

    pub fn get_orders_by_item(table:Table, item_id:String) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| OrderService::is_order_expired(order.clone()))
            .filter(|order| order.item_id.eq(item_id.as_str()))
            .collect::<Vec<Order>>();
        return orders;
    }

    pub fn get_orders_by_order_id(table:Table, order_id:String) -> Option<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| order.order_id.eq(order_id.as_str()))
            .next();
        // .collect::<Vec<Order>>();
        return orders;
    }

    pub fn to_json(order:Order) -> String {
        let mut json = String::from("{\n");
        json.push_str(&*format!("\t\"order_id\":\"{}\",\n", order.order_id));
        json.push_str(&*format!("\t\"table_id\":\"{}\",\n", order.table_id));
        json.push_str(&*format!("\t\"item_id\":\"{}\",\n", order.item_id));
        json.push_str(&*format!("\t\"cook_time\":{},\n", order.cook_time));
        json.push_str(&*format!("\t\"create_at\":{},\n", order.create_at.format("%Y/%m/%d %H:%M:%S")));
        json.push_str(&*String::from("},\n"));
        return json;
    }

    pub fn to_jsons(orders:Vec<Order>) -> String {
        let mut json = String::from("[\n");
        for order in orders {
            let tmp = OrderService::to_json(order);
            json.push_str(&*tmp);
        }
        json.push_str(&*String::from("]\n"));
        return json
    }

}
