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

    pub fn get_orders(table:Table, item_id:String) -> Vec<Order> {
        let orders = table.orders.clone();
        let orders = orders.into_iter()
            .filter(|order| !order.item_id.eq(item_id.as_str()))
            .collect::<Vec<Order>>();
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
            json.push_str(&*Order::get_json(order.clone()));
            // json = &*(json.to_owned() + Order::get_json(order.clone()));
        }
        return json;
    }

}

// pub(crate) struct Restaurant {
//     // change to vec<vec<Order>,
//     pub(crate) tables: Vec<Mutex<Vec<Order>>>,
//     // pub(crate) tables: HashMap<u32, Mutex<Vec<Order>>>,
// }
//
// impl Restaurant {
//     pub fn new(number_of_table:u32) -> Restaurant {
//         let mut tables = Vec::new();
//         Restaurant {
//             tables,
//         }
//     }
//
//     pub fn create_order(arc:Arc<Restaurant>, table_id:usize, item_id:String) -> Vec<Order> {
//         let mut orders = arc.tables[table_id].lock().unwrap().clone();
//         orders.push(Order::new(item_id));
//         return orders;
//     }
//
//     pub fn get_order(restaurant:Arc<Restaurant>, table_id:usize) -> Vec<Order> {
//         return  restaurant.tables[table_id].lock().unwrap().clone();
//     }
//
//     pub fn get_new_table() -> Mutex<Vec<Order>> {
//         return Mutex::new(Vec::new());
//     }
//
//     pub fn print(arc:Arc<Restaurant>) {
//         let tables = &arc.tables;
//         println!("**** Restaurant status ****");
//         println!("Total Orders: {}", tables.len());
//         for n in 0..arc.tables.len() {
//             println!("table: {} size: {}", n, arc.tables[n].lock().unwrap().len());
//         }
//     }
// }
