use std::borrow::Borrow;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::collections::HashMap;
use std::iter::Copied;
use std::slice::Iter;
use std::time::SystemTime;

use crate::order::Order;

pub(crate) struct Table {
    pub(crate) orders: Vec<Order>,
}
impl Clone for Table {
    fn clone(&self) -> Table {
        let mut new = Table::new();
        new.orders = self.orders.clone();
        return new;
    }
}

impl Table {
    pub fn new() -> Table {
        return Table {
            orders: Vec::new(),
        };
    }

    pub fn open_tables(number_of_tables:usize) -> Vec<Mutex<Table>> {
        let mut tables = Vec::new();
        for n in 0 .. number_of_tables {
            tables.push(Mutex::new(Table::new()));
        }
        return tables;
    }

    pub fn create_order(table:Table, item_id:String) -> Vec<Order> {
        let mut orders = table.orders.clone();
        orders.push(Order::new(item_id));
        return orders;
    }

    pub fn delete_order(table:Table, order_id:String) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| !order.order_id.eq(order_id.as_str()))
            .collect::<Vec<Order>>();
        println!("order size:{}",orders.len());
        return orders;
    }

    pub fn get_orders_active(table:Table) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| Order::is_order_expired(order.clone()))
            .collect::<Vec<Order>>();
        return orders;
    }

    pub fn get_orders_by_item(table:Table, item_id:String) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| !order.item_id.eq(item_id.as_str()))
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

    pub fn print(tables:Vec<Table>) {
        println!("**** Restaurant status ****");
        println!("Total Orders: {}", tables.len());
        for n in 0..tables.len() {
            println!("table: {} size: {}", n, tables[n].orders.len());
        }
    }

    pub fn to_json(table:Table) -> String {
        let mut json:String = "".to_string();
        for n in 0..table.orders.len() {
            let order = table.orders.get(n).unwrap();
            json.push_str(&*Order::to_json(order.clone()));
            // json = &*(json.to_owned() + Order::get_json(order.clone()));
        }
        return json;
    }
}
