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

pub struct Table {
    pub table_id: String,
    pub orders: Vec<Order>,
}
impl Clone for Table {
    fn clone(&self) -> Table {
        let mut new = Table::new(self.table_id.clone());
        new.orders = self.orders.clone();
        return new;
    }
}

impl Table {
    pub fn new(table_id: String) -> Table {
        return Table {
            table_id,
            orders: Vec::new(),
        };
    }

    // pub fn open_tables(number_of_tables:usize) -> Vec<Mutex<Table>> {
    //     let mut tables = Vec::new();
    //     for table_id in 0 .. number_of_tables {
    //         tables.push(Mutex::new(Table::new(table_id)));
    //     }
    //     return tables;
    // }

}
