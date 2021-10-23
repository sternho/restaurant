#[cfg(test)]
mod test_order_service {
    use crate::datetime_util;
    use crate::table::Table;
    use crate::order_service::OrderService;
    use crate::order::Order;

    static TIME_FORMAT:&str = "%Y/%m/%d %H:%M:%S";

    #[test]
    pub fn test_filter_item_id() {
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", TIME_FORMAT);
        let mut orders = Vec::new();
        orders.push(Order {
            order_id: "order_id2".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 2,
            create_at: order_time
        });
        orders.push(Order {
            order_id: "order_id2".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id2".to_string(),
            cook_time: 2,
            create_at: order_time
        });

        let mut filters = Vec::new();
        filters.push(OrderService::item_id_filter(String::from("item_id")));
        let updated_orders = OrderService::filter_orders(orders, filters);
        assert_eq!(1, updated_orders.len())
    }

    #[test]
    pub fn test_order_not_expired() {
        let mut orders = Vec::new();
        orders.push(Order {
            order_id: "order_id2".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 2,
            create_at: datetime_util::to_date_str(&"2021/10/14 18:59:00", TIME_FORMAT)
        });
        orders.push(Order {
            order_id: "order_id2".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id2".to_string(),
            cook_time: 2,
            create_at: datetime_util::to_date_str(&"2021/10/14 18:58:00", TIME_FORMAT)
        });

        let expire_time = datetime_util::to_date_str(&"2021/10/14 18:59:00", TIME_FORMAT);
        let mut filters = Vec::new();
        filters.push(OrderService::item_id_filter(String::from("item_id")));
        filters.push(OrderService::not_expired_filter(expire_time));
        let updated_orders = OrderService::filter_orders(orders, filters);
        assert_eq!(1, updated_orders.len())
    }

    #[test]
    pub fn test_is_able_create_order() {
        let mut orders = Vec::new();
        for i in 0..10 {
            orders.push("order".to_string()+ i.to_string().as_str())
        }
        let mut table = Table::new(String::from("table_1"));
        for i in 0..90 {
            table.orders.push("order".to_string()+ i.to_string().as_str())
        }

        let is_able = OrderService::is_able_create_order(table, orders);
        assert!(is_able)
    }

    #[test]
    pub fn test_is_able_create_order_false() {
        let mut orders = Vec::new();
        for i in 0..11 {
            orders.push("order".to_string()+ i.to_string().as_str())
        }
        let mut table = Table::new(String::from("table_1"));
        for i in 0..90 {
            table.orders.push("order".to_string()+ i.to_string().as_str())
        }

        let is_able = OrderService::is_able_create_order(table, orders);
        assert!(!is_able)
    }

}
