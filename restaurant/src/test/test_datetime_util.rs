#[cfg(test)]
mod test_datetime_util {
    use chrono::{Local, NaiveDateTime, TimeZone};
    use crate::datetime_util;

    #[test]
    pub fn to_date_str() {
        let date_str = &"2021/10/14 18:58:00";
        let format_str = &"%Y/%m/%d %H:%M:%S";
        let order_time = NaiveDateTime::parse_from_str(*date_str, *format_str).unwrap();
        let order_time = Local.from_local_datetime(&order_time).unwrap();

        let result = datetime_util::to_date_str(*date_str, *format_str);
        assert_eq!(order_time, result)
    }

    #[test]
    pub fn to_date() {
        let date_str = &"2021/10/14 18:58:00";
        let format_str = &"%Y/%m/%d %H:%M:%S";
        let order_time = NaiveDateTime::parse_from_str(*date_str, *format_str).unwrap();
        let order_time = Local.from_local_datetime(&order_time).unwrap();

        let result = datetime_util::to_date(String::from(*date_str), String::from(*format_str));
        assert_eq!(order_time, result)
    }

}