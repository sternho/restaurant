use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

/// translate the date string and format to DateTime Object
///
/// Example:
/// let datetime_obj = datetime_util::to_date_str(&"2021/10/14 18:58:00", &'%Y/%m/%d %H:%M:%S');
pub fn to_date_str(date: &str, format: &str) -> DateTime<Local> {
    let order_time = NaiveDateTime::parse_from_str(date, format).unwrap();
    let order_time = Local.from_local_datetime(&order_time).unwrap();
    order_time
}

/// use for handle serialize and deserialize datetime in JSON
pub mod json_date_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer, {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
        where D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        Local.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
