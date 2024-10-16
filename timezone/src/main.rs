use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use std::str::FromStr;

fn get_timezone(timezone_str: Option<String>) -> Result<Tz, Box<dyn std::error::Error>> {
    match timezone_str {
        Some(tz_str) => Tz::from_str(&tz_str).map_err(|e| e.into()),
        None => Tz::from_str("UTC").map_err(|e| e.into()), // 如果 timezone_str 为 None，则默认使用 UTC
    }
}

fn convert_to_timezone(
    datetime_str: &str,
    timezone_str: Option<String>,
) -> Result<DateTime<Tz>, String> {
    // 日期时间字符串-> NaiveDateTime
    let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Failed to parse datetime string: {}", e))?;
    let timezone =
        get_timezone(timezone_str).map_err(|e| format!("Failed to get timezone: {}", e))?;

    // 将 NaiveDateTime -> 指定时区的 DateTime
    let datetime_with_timezone = timezone
        .from_local_datetime(&naive_datetime)
        .single()
        .ok_or_else(|| "Ambiguous or non-existent local time".to_string())?;

    Ok(datetime_with_timezone)
}

fn main() {
    let datetime_str = "2024-06-24 12:23:23";
    let timezone_str = Some("Asia/Shanghai".to_string()); // 注意时区字符串格式

    match convert_to_timezone(datetime_str, timezone_str) {
        Ok(datetime_with_timezone) => {
            println!("Datetime in Asia/Shanghai: {}", datetime_with_timezone);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
