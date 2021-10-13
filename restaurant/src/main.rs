// extern crate restaurant;

// use restaurant_server::ThreadPool;
use std::{fs, mem};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::{Lines, Split, SplitN};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::order::Order;

use crate::table::{Table};
use crate::thread_pool::ThreadPool;

#[path = "http_server.rs"] mod thread_pool;
#[path = "table.rs"] mod table;
#[path = "order.rs"] mod order;

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

    let request = get_url(buffer);
    let api = request.get("api").unwrap();
    // println!("method: {}", request.get(0).unwrap());
    println!("Called API: {}", api);

    let mut html:String;
    // if compare(link,"/") {
    //     file_name = "resource/index.html";
    // } else
    if compare(api, "/create") {
        let table_id = request.get("table_id");
        let item_id = request.get("item_id");

        if table_id.is_some() && item_id.is_some() {
            let table_id = table_id.unwrap().parse::<usize>().unwrap();
            let mut table = tables[table_id].lock().unwrap();
            let orders = Table::create_order(table.clone(), item_id.unwrap().to_string());
            table.orders = orders;
            html = format!("order created successfully.");
        } else {
            html = format!("Please enter table_id and item_id.");
        }
    } else if compare(api, "/delete") {
        let table_id = request.get("table_id");
        let order_id = request.get("order_id");

        if table_id.is_some() && order_id.is_some() {
            let order_id = order_id.unwrap().to_string();
            let table_id = table_id.unwrap().parse::<usize>().unwrap();
            let mut table = tables[table_id].lock().unwrap();
            // let orders = Table::delete_order(table.clone(), order_id);
            // table.orders = orders;
            let order = Table::get_orders_by_order_id(table.clone(), order_id.clone());
            if order.is_some() {
                let orders = Table::delete_order(table.clone(), order_id.clone());
                table.orders = orders;
                html = format!("order delete successfully");
            } else {
                html = format!("Delete failure: order [{}] not found.", order_id);
            }
        } else {
            html = format!("Please enter table_id and order_id.");
        }
    } else if compare(api, "/check") {
        let table_id = request.get("table_id").unwrap().parse::<usize>().unwrap();
        let order_id = request.get("order_id");

        let mut table = tables[table_id].lock().unwrap();
        if order_id.is_some() {
            let order = Table::get_orders_by_order_id(table.clone(), order_id.unwrap().to_string());
            match order {
                Some(o) => {html = Order::to_json(o)}
                None => {html = format!("Order [{}] not found.", order_id.unwrap())}
            }
        } else {
            let orders = Table::get_orders_active(table.clone());
            html = Order::to_jsons(orders);
        }
    } else {
        status = 404;
        status_text = "not found";
        html = fs::read_to_string("resource/404.html").unwrap();
    }

    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", status, status_text, html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("request:\n{}", String::from_utf8_lossy(&buffer));
}

fn compare(url:&String, target:&str) -> bool {
    let tmp = String::from(target);
    url.starts_with(&tmp)
}

fn get_url(buffer:[u8;1024]) -> HashMap<String, String> {
    let url = get_request_para(buffer);
    let url = url.get(0).unwrap();
    let url = url.split(" ");
    let mut info = Vec::new();
    url.for_each(|x| info.push(x.to_string()));
    let mut para = HashMap::new();
    para.insert(String::from("method"), info.get(0).unwrap().to_string());

    let link = info.get(1).unwrap();
    let index = link.find("?").unwrap();
    let api:String = link.chars().take(index).collect();
    para.insert(String::from("api"), api);

    let request_para:String = link.chars().skip(index+1).collect();
    let request_para = request_para.split("&");
    request_para.for_each(|x| {
        let mut a = x.split("=");
        let key = a.next().unwrap().to_string();
        let value = a.next().unwrap().to_string();
        para.insert(key, value);
    });

    return para;
}

fn get_request_para(buffer:[u8;1024]) -> Vec<String> {
    let data = String::from_utf8(buffer.to_vec()).unwrap();
    let data = data.lines();
    let mut para = Vec::new();
    for x in data {
        let tmp = x.splitn(1,':');
        para.push(x.to_string());
    }
    return para;
}

fn path(location: &str) -> String {
    format!("GET {} HTTP/1.1\r\n", location)
}
