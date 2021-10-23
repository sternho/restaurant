use std::io::Cursor;
use chrono::Local;
use rocket::http::Status;
use rocket::Response;
use crate::table_redis;
use crate::table_service::TableService;
use crate::order_redis;

/// Handle the query table request when received.
/// receiving the table_id and item_id
/// pass to the TableService to receive table information and filter out correct orders.
/// And return the json format text outside.
///
/// sample request: [GET] http://localhost:8000/table/{table_id}
#[get("/<table_id>?<item_id>")]
pub fn table_query(table_id: String, item_id: Option<String>) -> Response<'static> {
    let table = table_redis::fetch(table_id);
    let response = TableService::parse_table(table, item_id, Local::now(),
                                             order_redis::fetch_orders);
    let json = serde_json::to_string(&response).unwrap();
    Response::build()
        .status(Status::Ok)
        .sized_body(Cursor::new(json))
        .finalize()
}
