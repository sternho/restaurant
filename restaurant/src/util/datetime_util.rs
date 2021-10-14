use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn to_date_str(date:&str, format:&str) -> DateTime<Local> {
    let order_time = NaiveDateTime::parse_from_str(date, format).unwrap();
    let order_time = Local.from_local_datetime(&order_time).unwrap();
    order_time
}

pub fn to_date(date:String, format:String) -> DateTime<Local> {
    to_date_str(date.as_str(), format.as_str())
}
