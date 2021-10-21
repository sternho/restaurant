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

/// translate the date string and format to DateTime Object
///
/// Example:
/// let datetime_obj = datetime_util::to_date(String::from('2021/10/14 18:58:00'), String::from('%Y/%m/%d %H:%M:%S'));
pub fn to_date(date: String, format: String) -> DateTime<Local> {
    to_date_str(date.as_str(), format.as_str())
}


pub mod json_date_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer, {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
        where D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        Local.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
