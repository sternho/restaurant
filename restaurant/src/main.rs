#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;

#[path = "api/table_api.rs"] mod table_api;
#[path = "api/order_api.rs"] mod order_api;
#[path = "util/datetime_util.rs"] mod datetime_util;
#[path = "dto/order_request.rs"] mod order_request;
#[path = "dto/table_response.rs"] mod table_response;
#[path = "dao/table.rs"] mod table;
#[path = "dao/order.rs"] mod order;
#[path = "repository/table_redis.rs"] mod table_redis;
#[path = "repository/order_redis.rs"] mod order_redis;
#[path = "service/order_service.rs"] mod order_service;
#[path = "service/table_service.rs"] mod table_service;
#[path = "../test/service/test_order_service.rs"] mod test_order_service;
#[path = "../test/service/test_table_service.rs"] mod test_table_service;
#[path = "../test/util/test_datetime_util.rs"] mod test_datetime_util;


/// main function to start the program
/// Creating the web server through Rocket
/// and do the request mapping
fn main() {
    rocket::ignite()
        .mount("/order", routes![order_api::order_create])
        .mount("/order", routes![order_api::order_query])
        .mount("/order", routes![order_api::order_delete])
        .mount("/table", routes![table_api::table_query])
        .launch();
}
