use crate::order::Order;

pub struct Table {
    pub table_id: String,
    pub orders: Vec<Order>,
}
impl Clone for Table {
    fn clone(&self) -> Table {
        let mut new = Table::new(self.table_id.clone());
        new.orders = self.orders.clone();
        return new;
    }
}

impl Table {
    pub fn new(table_id: String) -> Table {
        return Table {
            table_id,
            orders: Vec::new(),
        };
    }

}
