use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use chrono::Local;

use crate::action::Action;
use crate::order::Order;
use crate::order_service::OrderService;
use crate::table::Table;
use crate::thread_pool::ThreadPool;

#[path = "util/http_server.rs"] mod thread_pool;
#[path = "util/datetime_util.rs"] mod datetime_util;
#[path = "dao/table.rs"] mod table;
#[path = "dao/order.rs"] mod order;
#[path = "repository/redis_handler.rs"] mod redis_handler;
#[path = "service/order_service.rs"] mod order_service;
#[path = "action.rs"] mod action;
#[path = "test/test_order_service.rs"] mod test_order_service;
#[path = "test/test_datetime_util.rs"] mod test_datetime_util;

static ADDRESS: &str = "127.0.0.1:3000";
static THREAD_POOL: usize = 10;

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    let thread_pool = ThreadPool::new(THREAD_POOL);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| handle_connection(stream));
    }

    println!("shutting down");
}

fn handle_connection(mut stream:TcpStream) {
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
        let item_id = request.get("item_id");
        let order_id = request.get("order_id");
        let table_id = request.get("table_id").unwrap().to_string();
        let table = redis_handler::fetch_table(table_id);

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
        html = format!("Please enter all mandatory fills.");
    }

    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", status, status_text, html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("request:\n{}", String::from_utf8_lossy(&buffer));
}

fn delete_action(mut table:Table, order_id:String) -> String {
    let order = OrderService::get_orders_by_order_id(table.clone(), order_id.clone());
    return if order.is_some() {
        let orders = OrderService::delete_order(table.clone(), order_id.clone());
        table.orders = orders;
        redis_handler::put_table(table);
        format!("Order delete successfully")
    } else {
        format!("Delete failure: order [{}] not found.", order_id)
    }
}

fn create_action(mut table:Table, item_id:String) -> String {
    let item: Vec<String> = item_id.split(",")
        .map(|s| s.to_string()).collect();

    if !OrderService::is_too_much_order(table.clone(), item.len()) {
        let orders = OrderService::create_order(table.clone(), item);
        table.orders = orders;
        redis_handler::put_table(table);
        format!("Order created successfully.")
    } else {
        format!("This table got too much orders.")
    }
}

fn query_action(table:Table, item_id:Option<&String>) -> String {
    let orders;
    if item_id.is_some() {
        orders = OrderService::get_orders_by_item(table.clone(),
                                                  item_id.unwrap().to_string(),
                                                  Local::now());
    } else {
        orders = OrderService::get_orders_active(table.clone(), Local::now());
    }
    let html = OrderService::to_jsons(orders);
    html
}
