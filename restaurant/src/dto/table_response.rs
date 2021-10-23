use serde::{Deserialize, Serialize};
use crate::order::Order;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TableResponse {
    pub table_id: String,
    pub orders: Vec<Order>,
}
