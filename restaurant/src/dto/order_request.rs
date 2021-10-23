use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCreateRequest {
    pub table_id: String,
    pub item_id: Vec<String>,
}
