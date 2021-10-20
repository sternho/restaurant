use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

/// translate the date string and format to DateTime Object
///
/// Example:
/// let datetime_obj = datetime_util::to_date_str(&"2021/10/14 18:58:00", &'%Y/%m/%d %H:%M:%S');
pub fn to_date_str(date:&str, format:&str) -> DateTime<Local> {
    let order_time = NaiveDateTime::parse_from_str(date, format).unwrap();
    let order_time = Local.from_local_datetime(&order_time).unwrap();
    order_time
}

/// translate the date string and format to DateTime Object
///
/// Example:
/// let datetime_obj = datetime_util::to_date(String::from('2021/10/14 18:58:00'), String::from('%Y/%m/%d %H:%M:%S'));
pub fn to_date(date:String, format:String) -> DateTime<Local> {
    to_date_str(date.as_str(), format.as_str())
}
