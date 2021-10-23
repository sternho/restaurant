/// define table struct and relate member variable.
pub struct Table {
    pub table_id: String,
    pub orders: Vec<String>,
}

/// for clone the table object.
///
/// Example:
/// let new_object = table.clone();
impl Clone for Table {
    fn clone(&self) -> Table {
        let mut new_table = Table::new(self.table_id.clone());
        new_table.orders = self.orders.clone();
        return new_table;
    }
}

impl Table {
    /// Table constructor
    /// Create Table Object by table_id
    ///
    /// Example:
    /// let table = Table::new(String::from("table1"));
    pub fn new(table_id: String) -> Table {
        return Table {
            table_id,
            orders: Vec::new(),
        };
    }
}
