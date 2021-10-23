use std::io::Cursor;

use rand::Rng;
use rocket::http::Status;
use rocket::Response;

use crate::order_redis;
use crate::order_request::OrderCreateRequest;
use crate::order_service::OrderService;
use crate::table_service::TableService;

/// handle the delete order request when received.
/// received the input parameter and pass to the OrderService to handle.
/// After get back the result, pass the result to redis_handler save.
///
/// sample request: [DELETE] http://localhost:8000/order/{order_id}
/// body:
#[delete("/<order_id>")]
pub fn order_delete(order_id: String) -> Response<'static> {
    let order = order_redis::fetch(order_id.clone());
    if order.is_some() {
        OrderService::delete_order(order.unwrap());
        // format!("Order delete successfully")
        Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new("Order delete successfully"))
            .finalize()
    } else {
        Response::build()
            .status(Status::NotFound)
            .sized_body(Cursor::new(format!("Delete failure: order [{}] not found.", order_id)))
            .finalize()
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
pub fn order_create(order_json: String) -> Response<'static> {
    let order_request: OrderCreateRequest = serde_json::from_str(&order_json).unwrap();
    let table = TableService::get_table(order_request.table_id.clone());
    if OrderService::is_able_create_order(table, order_request.item_id.clone()) {
        let cook_time = rand::thread_rng().gen_range(5..15);
        let orders = OrderService::create_order(order_request.table_id.clone(),
                                                order_request.item_id.clone(), cook_time);
        OrderService::save_order(orders.clone());
        let json = serde_json::to_string(&orders).unwrap();
        Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(json))
            .finalize()
    } else {
        Response::build()
            .status(Status::BadRequest)
            .sized_body(Cursor::new("This table got too much orders"))
            .finalize()
    }
}

/// Handle the query order request when received.
/// received the input parameter and pass to the OrderService to filter out correct orders.
/// And return the json format text outside.
///
/// sample request: [GET] http://localhost:8000/order/{order_id}
///
#[get("/<order_id>")]
pub fn order_query(order_id: String) -> Response<'static> {
    // repository call back service.
    let order = order_redis::fetch(order_id.clone());
    return if order.is_some() {
        let order = order.unwrap();
        let json = serde_json::to_string(&order).unwrap();

        Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(json))
            .finalize()
    } else {
        Response::build()
            .status(Status::NotFound)
            .sized_body(Cursor::new(format!("Order [{}] cannot be found.", order_id)))
            .finalize()
    };
}
