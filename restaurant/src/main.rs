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

use crate::restaurant::{Table};
use crate::thread_pool::ThreadPool;

#[path = "http_server.rs"] mod thread_pool;
#[path = "restaurant.rs"] mod restaurant;
#[path = "order.rs"] mod order;

static ADDRESS: &str = "127.0.0.1:3000";
static THREAD_POOL: usize = 10;

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    let thread_pool = ThreadPool::new(THREAD_POOL);
    // let restaurant = Arc::new(Restaurant::new(10));
    let tables = Arc::new(Table::open_tables(10));

    for stream in listener.incoming() {
        // let restaurant = restaurant.clone();
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
    // } else if compare(link, "/sleep") {
    //     thread::sleep(Duration::from_secs(10));
    //     file_name = "resource/sleep.html"
    // } else
    if compare(api, "/create") {
        let table_id = request.get("table_id").unwrap().parse::<usize>().unwrap();
        let item_id = request.get("item_id").unwrap().to_string();

        let mut table = tables[table_id].lock().unwrap();
        let orders = Table::create_order(table.clone(), item_id);
        table.orders = orders;
        html = String::from("order created sucessfully");
    } else if compare(api, "/delete") {
        let table_id = request.get("table_id").unwrap().parse::<usize>().unwrap();
        let order_id = request.get("order_id").unwrap().to_string();

        let mut table = tables[table_id].lock().unwrap();
        let orders = Table::delete_order(table.clone(), order_id);
        table.orders = orders;
        html = String::from("order delete sucessfully");
    } else if compare(api, "/check") {
        let table_id = request.get("table_id").unwrap().parse::<usize>().unwrap();
        let item_id = request.get("item_id").unwrap().to_string();

        let mut table = tables[table_id].lock().unwrap();
        html = Table::to_json(table.clone()).to_string();
    } else {
        status = 404;
        status_text = "not found";
        html = fs::read_to_string("resource/404.html").unwrap();
    }

    // if buffer.starts_with(path("/").as_bytes()) {
    //     file_name = "resource/index.html";
    // } else if buffer.starts_with(path("/sleep").as_bytes()) {
    //     thread::sleep(Duration::from_secs(10));
    //     file_name = "resource/sleep.html"
    // } else if buffer.starts_with(path("/create").as_bytes()) {
    //     let table_id:u32 = 0;
    //     let item_id = String::from("item1");
    //
    //     let mut table = tables[0].lock().unwrap();
    //     let orders = Table::create_order(table.clone(), item_id);
    //     table.orders = orders;
    //     println!("order created sucessfully");
    //
    //     file_name = "resource/index.html";
    // } else if buffer.starts_with(path("/check").as_bytes()) {
    //     // Restaurant::print(restaurant);
    //     let mut tmp = Vec::new();
    //     for n in 0 .. tables.len() {
    //         tmp.push(tables[n].lock().unwrap().clone());
    //     }
    //     Table::print(tmp);
    //
    //     file_name = "resource/index.html";
    // } else {
    //     status = 404;
    //     status_text = "not found";
    //     file_name = "resource/404.html";
    // }

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
