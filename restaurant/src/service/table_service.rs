use chrono::{DateTime, Local};
use crate::order::Order;
use crate::table::Table;
use crate::table_response::TableResponse;
use crate::order_service::OrderService;
use crate::table_redis;

pub struct TableService {}

/// max order can be place for a table
static MAX_ORDER_NUMBER: usize = 100;

impl TableService {
    /// use to get active orders and combine it into Table Response format
    /// if item_id is available, it also would filter out the specific items
    ///
    /// Examples:
    /// let table_response = parse_table(table, item_id, order_redis::fetch_orders);
    pub fn parse_table(table: Table, item_id: Option<String>, expired: DateTime<Local>,
                       get_orders: fn(Vec<String>) -> Vec<Order>) -> TableResponse {
        let orders = get_orders(table.orders);
        let mut filters = vec![OrderService::not_expired_filter(expired)];
        if item_id.is_some() {
            filters.push(OrderService::item_id_filter(item_id.unwrap()));
        }
        let orders = OrderService::filter_orders(orders, filters);
        TableResponse {
            table_id: table.table_id,
            orders,
        }
    }

    /// Use to check the order limit for table.
    /// return false if the existing orders + new orders is larger than the limitation
    ///
    /// Examples:
    /// let over_limit = is_too_much_order(table, 10);
    pub fn is_too_much_order(table: Table, new_item_cnt: usize) -> bool {
        table.orders.len() + new_item_cnt > MAX_ORDER_NUMBER
    }

    /// get table by table_id from redis
    ///
    /// Examples:
    /// let table = get_table(String::from("table_id"))
    pub fn get_table(table_id:String) -> Table {
        table_redis::fetch(table_id)
    }

}