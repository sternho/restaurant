#[cfg(test)]
mod test_order_service {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
    use crate::datetime_util;
    use crate::table::Table;
    use super::*;
    use crate::order_service::OrderService;
    use crate::order::Order;

    static time_format:&str = "%Y/%m/%d %H:%M:%S";

    #[test]
    pub fn test_filter() {
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", time_format);
        let table_id = String::from("table_id");
        let mut table = Table::new(table_id.clone());

        let mut orders = Vec::new();
        orders.push(Order {
            order_id: "order_id1".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 1,
            create_at: order_time
        });
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
        table.orders = orders;

        let expire_time = datetime_util::to_date_str(&"2021/10/14 18:59:00", time_format);
        let mut filters = Vec::new();
        filters.push(OrderService::item_id_filter(String::from("item_id")));
        filters.push(OrderService::order_id_filter(String::from("order_id2")));
        filters.push(OrderService::expired_filter(expire_time));
        let updated_orders = OrderService::filter_orders(table, filters);
        assert_eq!(1, updated_orders.len())
    }

    #[test]
    pub fn test_is_order_expired() {
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", time_format);
        let order = Order {
            order_id: "order_id".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 2,
            create_at: order_time
        };

        let expire_time = datetime_util::to_date_str(&"2021/10/14 18:59:00", time_format);
        let expired = OrderService::is_order_expired(order, expire_time);
        assert!(expired)
    }

    #[test]
    pub fn test_is_order_expired_false() {
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", time_format);
        let order = Order {
            order_id: "order_id".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 1,
            create_at: order_time
        };

        let expire_time = datetime_util::to_date_str(&"2021/10/14 18:59:00", time_format);
        let expired = OrderService::is_order_expired(order, expire_time);
        assert!(!expired)
    }

    #[test]
    pub fn test_is_too_much_order() {
        let table_id = String::from("table_id");
        let mut table = Table::new(table_id.clone());
        for i in 0..99 {
            table.orders.push(Order::new(table_id.clone(), format!("item{}", i)));
        }
        let too_much = OrderService::is_too_much_order(table, 2);
        assert!(too_much)
    }

    #[test]
    pub fn test_is_too_much_order_false() {
        let table_id = String::from("table_id");
        let mut table = Table::new(table_id.clone());
        for i in 0..99 {
            table.orders.push(Order::new(table_id.clone(), format!("item{}", i)));
        }
        let too_much = OrderService::is_too_much_order(table, 1);
        assert!(!too_much)
    }

    #[test]
    pub fn test_create_order() {
        let table_id = String::from("table_id");
        let mut table = Table::new(table_id.clone());

        let mut items = Vec::new();
        items.push(String::from("item1"));
        items.push(String::from("item2"));

        let updated_orders = OrderService::create_order(table, items);
        assert_eq!(2, updated_orders.len())
    }

    #[test]
    pub fn test_delete_order() {
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", time_format);
        let table_id = String::from("table_id");
        let mut table = Table::new(table_id.clone());

        let mut orders = Vec::new();
        orders.push(Order {
            order_id: "order_id1".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 2,
            create_at: order_time
        });
        orders.push(Order {
            order_id: "order_id2".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 2,
            create_at: order_time
        });
        table.orders = orders;

        let updated_orders = OrderService::delete_order(table, String::from("order_id1"));
        assert_eq!(1, updated_orders.len())
    }

    #[test]
    pub fn test_to_json() {
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", time_format);

        let order = Order {
            order_id: "order_id2".to_string(),
            table_id: "table_id".to_string(),
            item_id: "item_id".to_string(),
            cook_time: 2,
            create_at: order_time
        };
        let json = OrderService::to_json(order);
        let expired = String::from("{\n\t\"order_id\":\"order_id2\",\n\t\"table_id\":\"table_id\",\
        \n\t\"item_id\":\"item_id\",\n\t\"cook_time\":2,\n\t\"create_at\":2021/10/14 18:58:00,\n},\n");
        assert_eq!(expired, json)
    }

    #[test]
    pub fn test_to_jsons() {
        let order_time = datetime_util::to_date_str(&"2021/10/14 18:58:00", time_format);

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
            item_id: "item_id".to_string(),
            cook_time: 2,
            create_at: order_time
        });
        let json = OrderService::to_jsons(orders);
        let expired = String::from("[\n{\n\t\"order_id\":\"order_id2\",\n\t\"table_id\":\"table_id\",\
                \n\t\"item_id\":\"item_id\",\n\t\"cook_time\":2,\n\t\"create_at\":2021/10/14 18:58:00,\n},\n\
                {\n\t\"order_id\":\"order_id2\",\n\t\"table_id\":\"table_id\",\n\t\"item_id\":\"item_id\",\
                \n\t\"cook_time\":2,\n\t\"create_at\":2021/10/14 18:58:00,\n},\n]\n");
        assert_eq!(expired, json)
    }

}
