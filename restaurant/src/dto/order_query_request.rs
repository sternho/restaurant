
use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderQueryRequest {
    pub table_id: String,
    pub item_id: Option<String>,
}
