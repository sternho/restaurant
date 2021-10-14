// use std::{fs, mem};
use std::fs;
// use std::cell::RefCell;
// use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// use std::str::{Lines};
use std::sync::{Arc, Mutex, MutexGuard};
// use std::thread;
// use std::time::Duration;

use crate::action::Action;
use crate::order::Order;
use crate::table::Table;
use crate::thread_pool::ThreadPool;

#[path = "http_server.rs"] mod thread_pool;
#[path = "table.rs"] mod table;
#[path = "order.rs"] mod order;
#[path = "action.rs"] mod action;

static ADDRESS: &str = "127.0.0.1:3000";
static THREAD_POOL: usize = 10;

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    let thread_pool = ThreadPool::new(THREAD_POOL);
    let tables = Arc::new(Table::open_tables(10));

    for stream in listener.incoming() {
        let tables = tables.clone();

        let stream = stream.unwrap();
        thread_pool.execute(|| handle_connection(stream, tables));
    }

    println!("shutting down");
}

fn handle_connection(mut stream:TcpStream, tables:Arc<Vec<Mutex<Table>>>) {
    let status = 200;
    let status_text = "ok";
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = Action::get_url_and_parameter(buffer);
    let api = request.get("api").unwrap();
    println!("Called API: {}", api);

    let html:String;
    let (action, check) = Action::action_parse(request.clone());
    if Action::None == action {
        html = fs::read_to_string("resource/html/index.html").unwrap();
    } else if check {
        let table_id = request.get("table_id").unwrap();
        let table_id = table_id.parse::<usize>().unwrap();
        if table_id < tables.len() {
            let item_id = request.get("item_id");
            let order_id = request.get("order_id");

            let table = tables[table_id].lock().unwrap();
            match action {
                Action::Create => {
                    let item_id = item_id.unwrap().to_string();
                    html = create_action(table, item_id);
                },
                Action::Delete => {
                    let order_id = order_id.unwrap().to_string();
                    html = delete_action(table, order_id);
                },
                Action::Query => {
                    html = query_action(table, item_id);
                },
                _ => {
                    html = fs::read_to_string("../resource/html/index.html").unwrap();
                }
            }
        } else {
            html = format!("Table number is not correct. Please enter 0 to {}", (tables.len()-1));
        }
    } else {
        html = format!("Please enter all mandatory fills.");
    }

    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", status, status_text, html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("request:\n{}", String::from_utf8_lossy(&buffer));
}

fn delete_action(mut table:MutexGuard<Table>, order_id:String) -> String {
    let order = Table::get_orders_by_order_id(table.clone(), order_id.clone());
    return if order.is_some() {
        let orders = Table::delete_order(table.clone(), order_id.clone());
        table.orders = orders;
        // let mut temp = table.clone();
        // temp.orders = orders.clone();
        format!("Order delete successfully")
    } else {
        format!("Delete failure: order [{}] not found.", order_id)
    }
}

fn create_action(mut table:MutexGuard<Table>, item_id:String) -> String {
    if !Table::is_too_much_order(table.clone()) {
        let orders = Table::create_order(table.clone(), item_id);
        table.orders = orders;
        format!("Order created successfully.")
    } else {
        format!("This table got too much orders.")
    }
}

fn query_action(table:MutexGuard<Table>, item_id:Option<&String>) -> String {
    let orders;
    if item_id.is_some() {
        orders = Table::get_orders_by_item(table.clone(), item_id.unwrap().to_string());
    } else {
        orders = Table::get_orders_active(table.clone());
    }
    let html = Order::to_jsons(orders);
    html
}
