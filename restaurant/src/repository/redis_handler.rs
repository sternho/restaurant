extern crate redis;
use crate::order::Order;
use crate::table::Table;

use chrono::{Local, NaiveDateTime, TimeZone};
use redis::Commands;

static REDIS_ADDRESS: &str = "redis://127.0.0.1/";
static REDIS_DATETIME_FORMAT:&str = "%Y/%m/%d %H:%M:%S";

/// Save the table object to redis
/// redis key: table_id
pub fn put_table(table:Table) {
    let serialized = serialize_table(table.clone());

    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();
    let _ : () = con.set(table.table_id, serialized).unwrap();
}

/// Pull out table object from redis by table_id
pub fn fetch_table(table_id:String) -> Table {
    let client = redis::Client::open(REDIS_ADDRESS);
    let mut con = client.unwrap().get_connection().unwrap();

    let result = con.get(table_id.clone());
    let table:Table;
    if result.is_ok() {
        table = deserialize_table(result.unwrap());
    } else {
        table = Table::new(table_id);
    }

    return table;
}

/// Parse the table object to string
///
/// Example:
/// Input table object:
/// table {
///     table_id: 'table1',
///     order: {
///         order_id: 'order1',
///         table_id: 'table1',
///         item_id: 'item1',
///         cook_time: 5
///         create_at: '2021/10/14 18:58:00'
///     }
/// }
/// Output Redis format:
/// order1|table1|item1|5|2021/10/14 18:58:00
fn serialize_table(table:Table) -> String{
    let orders = table.orders;
    let mut result = String::from("");
    for order in orders {
        result.push_str(&*order.order_id.to_string());
        result.push_str("|");
        result.push_str(&*order.table_id.to_string());
        result.push_str("|");
        result.push_str(&*order.item_id);
        result.push_str("|");
        result.push_str(&*order.cook_time.to_string());
        result.push_str("|");
        result.push_str(&*order.create_at.format(REDIS_DATETIME_FORMAT).to_string());
        result.push_str(",");
    }
    result
}


/// Parse the table object from string
///
/// Example:
/// Input Redis formatted String:
/// order1|table1|item1|5|2021/10/14 18:58:00
///
/// Output table object:
/// table {
///     table_id: 'table1',
///     order: {
///         order_id: 'order1',
///         table_id: 'table1',
///         item_id: 'item1',
///         cook_time: 5
///         create_at: '2021/10/14 18:58:00'
///     }
/// }
fn deserialize_table(string:String) -> Table{
    let mut tmp_id = String::from("");
    let list = string.split(",");
    let mut orders = Vec::new();
    list.for_each(|data| if !data.eq("") {
        let mut items = data.split("|");
        let order_id = items.next().unwrap().to_string();
        let table_id = items.next().unwrap().to_string();
        let item_id = items.next().unwrap().to_string();
        let cook_time = items.next().unwrap().parse::<usize>().unwrap();
        let tmp = items.next().unwrap().to_string();
        let tmp = NaiveDateTime::parse_from_str(&*tmp, REDIS_DATETIME_FORMAT).unwrap();
        let tmp = Local.from_local_datetime(&tmp).unwrap();
        let create_at = tmp;
        let order = Order {
            order_id,
            table_id: table_id.clone(),
            item_id,
            cook_time,
            create_at
        };
        tmp_id = table_id;
        orders.push(order);
    });
    let mut table = Table::new(tmp_id);
    table.orders = orders;
    return table;
}

