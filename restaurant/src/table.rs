// use std::borrow::Borrow;
// use std::collections::HashMap;
// use std::iter::Copied;
// use std::slice::Iter;
// use std::sync::{Arc, Mutex, MutexGuard};
// use std::thread;
// use std::time::SystemTime;
use std::sync::Mutex;

use crate::order::Order;

static MAX_ORDER_NUMBER: usize = 100;

pub(crate) struct Table {
    pub(crate) table_id: usize,
    pub(crate) orders: Vec<Order>,
}
impl Clone for Table {
    fn clone(&self) -> Table {
        let mut new = Table::new(self.table_id);
        new.orders = self.orders.clone();
        return new;
    }
}

impl Table {
    pub fn new(table_id: usize) -> Table {
        return Table {
            table_id,
            orders: Vec::new(),
        };
    }

    pub fn open_tables(number_of_tables:usize) -> Vec<Mutex<Table>> {
        let mut tables = Vec::new();
        for table_id in 0 .. number_of_tables {
            tables.push(Mutex::new(Table::new(table_id)));
        }
        return tables;
    }

    pub fn is_too_much_order(table:Table) -> bool {
        table.orders.len() > MAX_ORDER_NUMBER
    }

    pub fn create_order(table:Table, item_id:String) -> Vec<Order> {
        let mut orders = table.orders.clone();
        orders.push(Order::new(table.table_id, item_id));
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
            .filter(|order| Order::is_order_expired(order.clone()))
            .collect::<Vec<Order>>();
        return orders;
    }

    pub fn get_orders_by_item(table:Table, item_id:String) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| Order::is_order_expired(order.clone()))
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
