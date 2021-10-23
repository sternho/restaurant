#[cfg(test)]
mod test_table_service {
    use crate::datetime_util;
    use crate::order::Order;
    use crate::table::Table;
    use crate::table_service::TableService;

    static TIME_FORMAT:&str = "%Y/%m/%d %H:%M:%S";

    #[test]
    pub fn test_parse_table() {
        let mut table = Table::new(String::from("table1"));
        table.orders.push(String::from("item1"));
        table.orders.push(String::from("item2"));
        let response = TableService::parse_table(table,
                                                 Option::from(String::from("item1")),
                                                 datetime_util::to_date_str(&"2021/10/14 18:59:00", TIME_FORMAT),
                                                 mock_orders);
        assert_eq!(1, response.orders.len())
    }

    pub fn mock_orders(order: Vec<String>) -> Vec<Order> {
        println!("{}", order.len());
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", TIME_FORMAT);
        let mut orders = Vec::new();
        orders.push(Order {
            order_id: "order1".to_string(),
            table_id: "table1".to_string(),
            item_id: "item1".to_string(),
            cook_time: 2,
            create_at: order_time
        });
        orders.push(Order {
            order_id: "order2".to_string(),
            table_id: "table1".to_string(),
            item_id: "item2".to_string(),
            cook_time: 2,
            create_at: order_time
        });
        orders
    }

    #[test]
    pub fn test_is_too_much_order() {
        let table_id = String::from("table_id");
        let mut table = Table::new(table_id.clone());
        for i in 0..99 {
            table.orders.push("order".to_string()+ i.to_string().as_str());
        }
        let too_much = TableService::is_too_much_order(table, 2);
        assert!(too_much)
    }

    #[test]
    pub fn test_is_too_much_order_false() {
        let table_id = String::from("table_id");
        let mut table = Table::new(table_id.clone());
        for i in 0..99 {
            table.orders.push("order".to_string()+ i.to_string().as_str());
        }
        let too_much = TableService::is_too_much_order(table, 1);
        assert!(!too_much)
    }
}