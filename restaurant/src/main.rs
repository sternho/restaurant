#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;

use chrono::Local;
use rand::Rng;

use crate::order_query_request::OrderQueryRequest;
use crate::order_request::OrderCreateRequest;
use crate::order_service::OrderService;

#[path = "util/datetime_util.rs"] mod datetime_util;
#[path = "dto/order_request.rs"] mod order_request;
#[path = "dto/order_query_request.rs"] mod order_query_request;
#[path = "dao/table.rs"] mod table;
#[path = "dao/order.rs"] mod order;
#[path = "repository/redis_handler.rs"] mod redis_handler;
#[path = "service/order_service.rs"] mod order_service;
#[path = "test/test_order_service.rs"] mod test_order_service;
#[path = "test/test_datetime_util.rs"] mod test_datetime_util;

/// main function to start the program
/// Creating threads to receive and handle HTTP request.
fn main() {
    rocket::ignite()
        .mount("/order", routes![order_create])
        .mount("/order", routes![order_query])
        .mount("/order", routes![order_delete])
        .launch();
}

/// handle the delete order request when received.
/// received the input parameter and pass to the OrderService to handle.
/// After get back the result, pass the result to redis_handler save.
///
/// sample request: [DELETE] http://localhost:8000/order?order_id={}&table_id={}
/// body:
#[delete("/?<table_id>&<order_id>")]
fn order_delete(table_id:String, order_id: String) -> String {
    let mut table = redis_handler::fetch_table(table_id);

    let filter = vec![OrderService::order_id_filter(order_id.clone())];
    let order = OrderService::filter_orders(table.clone(),filter);
    return if order.len()>0 {
        let orders = OrderService::delete_order(table.clone(), order_id.clone());
        table.orders = orders;
        redis_handler::put_table(table);
        format!("Order delete successfully")
    } else {
        format!("Delete failure: order [{}] not found.", order_id)
    }
}

/// handle the create order request when received.
/// received the input parameter and pass to the OrderService to handle.
/// After get back the result, pass the result to redis_handler save.
///
/// sample request: [POST] http://localhost:8000/order
/// body:
/// {
///     "table_id": "table1",
///     "item_id": ["item1", "item2"]
/// }
#[post("/", data = "<order_json>")]
fn order_create(order_json: String) -> String {
    let order_request:OrderCreateRequest = serde_json::from_str(&order_json).unwrap();
    let mut table = redis_handler::fetch_table(order_request.table_id);

    if !OrderService::is_too_much_order(table.clone(), order_request.item_id.len()) {
        let cook_time = rand::thread_rng().gen_range(5..15);
        let orders = OrderService::create_order(table.clone(), order_request.item_id, cook_time);
        table.orders = orders;
        redis_handler::put_table(table);
        format!("Order created successfully.")
    } else {
        format!("This table got too much orders.")
    }
}

/// handle the query order request when received.
/// received the input parameter and pass to the OrderService to fitler out correct orders.
/// And return the json format text outside.
///
/// sample request: [GET] http://localhost:8000/order
/// body:
/// {
///     "table_id": "table1",
///     "item_id": "item2"
/// }
#[get("/", data = "<query_json>")]
fn order_query(query_json: String) -> String {
    let query:OrderQueryRequest = serde_json::from_str(&query_json).unwrap();
    let table = redis_handler::fetch_table(query.table_id);

    let mut filters = vec![OrderService::expired_filter(Local::now())];
    if query.item_id.is_some() {
        filters.push(OrderService::item_id_filter(query.item_id.unwrap()));
    }
    let orders = OrderService::filter_orders(table, filters);
    let json = OrderService::to_jsons(orders);
    json
}