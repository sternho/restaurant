extern crate redis;
use crate::order::Order;
use crate::table::Table;

use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use redis::{Commands, RedisResult};
use self::redis::Connection;

fn get_connection() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1/");
    let con = client.unwrap().get_connection().unwrap();
    return con;
}

// pub fn put_number(table_id:i32) {
//     let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//     let mut con = client.get_connection().unwrap();
//     let _ : () = con.set(table_id, 42).unwrap();
// }

pub fn put_table(table:Table) {
    let serialized = serialize_table(table.clone());

    let client = redis::Client::open("redis://127.0.0.1/");
    let mut con = client.unwrap().get_connection().unwrap();
    let _ : () = con.set(table.table_id, serialized).unwrap();
}

pub fn fetch_table(table_id:String) -> Table {
    // let mut con = get_connection();
    // con.get(table_id)
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    // let result = con.get(table_id).unwrap();
    // let result = deserialize_table(result);

    let result = con.get(table_id.clone());
    let mut table:Table;
    if result.is_ok() {
        table = deserialize_table(result.unwrap());
    } else {
        table = Table::new(table_id);
    }

    // let result = con.get(table_id, (e, data) => {
    //     if e {
    //         deserialize_table(result);
    //     } else {
    //         Table::new(table_id.clone());
    //     }
    // });
    return table;
}

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
        result.push_str(&*order.create_at.format("%Y/%m/%d %H:%M:%S").to_string());
        result.push_str(",");
    }
    result
}

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
        let tmp = NaiveDateTime::parse_from_str(&*tmp, "%Y/%m/%d %H:%M:%S").unwrap();
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

