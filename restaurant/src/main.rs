// extern crate restaurant;

// use restaurant_server::ThreadPool;
use std::{fs, mem};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::{Lines, Split, SplitN};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

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

fn handle_connection(mut stream:TcpStream, mut tables:Arc<Vec<Mutex<Table>>>) {
    // let file_name;
    let mut status = 200;
    let mut status_text = "ok";
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = Action::get_url(buffer);
    let api = request.get("api").unwrap();
    println!("Called API: {}", api);

    let mut html:String;

    let (action, check) = Action::action_parse(request.clone());
    if check {
        let table_id = request.get("table_id");
        let item_id = request.get("item_id");
        let order_id = request.get("order_id");

        let table_id = table_id.unwrap().parse::<usize>().unwrap();
        let mut table = tables[table_id].lock().unwrap();

        match action {
            Action::None => {
                html = fs::read_to_string("resource/index.html").unwrap();
            },
            Action::Create => {
                html = create_action(table, item_id.unwrap().to_string());
            },
            Action::Delete => {
                let order_id = order_id.unwrap().to_string();
                html = delete_action(table, order_id);
            },
            Action::Query => {
                html = query_action(table, order_id, item_id);
            },
        }
    } else {
        html = format!("Please enter mandatory fill.");
    }

    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", status, status_text, html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("request:\n{}", String::from_utf8_lossy(&buffer));
}

fn delete_action(mut table:MutexGuard<Table>, order_id:String) -> String {
    let order = Table::get_orders_by_order_id(table.clone(), order_id.clone());
    return if order.is_some() {
        let orders = Table::delete_order(table.clone(), order_id.clone());
        table.orders = orders;
        format!("order delete successfully")
    } else {
        format!("Delete failure: order [{}] not found.", order_id)
    }
}

fn create_action(mut table:MutexGuard<Table>, item_id:String) -> String {
    let orders = Table::create_order(table.clone(), item_id);
    table.orders = orders;
    format!("order created successfully.")
}

fn query_action(table:MutexGuard<Table>, order_id:Option<&String>, item_id:Option<&String>) -> String {
    let html:String;
    if order_id.is_some() {
        let order = Table::get_orders_by_order_id(table.clone(), order_id.unwrap().to_string());
        match order {
            Some(o) => { html = Order::to_json(o) }
            None => { html = format!("Order [{}] not found.", order_id.unwrap()) }
        }
    } else {
        let orders = Table::get_orders_active(table.clone());
        html = Order::to_jsons(orders);
    }
    html
}
