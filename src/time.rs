use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, NaiveDateTime, TimeZone, Local};
use chrono_tz::Tz;
use anyhow::Result;

/// 获取当前时间戳，单位秒
///
/// Examples
///
/// ```
/// use lazytool::time;
///
/// let ts = time::current_timestamp();
/// assert!(ts > 1736838663);
///
/// ```
pub fn current_timestamp() -> u64 {
    let start = SystemTime::now();

    // 计算自 UNIX 纪元以来的持续时间
    let duration = start.duration_since(UNIX_EPOCH)
        .expect("时间错误");

    // 获取时间戳（秒和毫秒）
    duration.as_secs()
}

/// 通过时间字符串解析得到 `DateTime<Local>` 结构体
///
/// Examples
///
/// ```
/// use lazytool::time;
///
/// let dt = time::from_str("2025-01-15 18:16:13", "%Y-%m-%d %H:%M:%S").unwrap();
///
/// assert_eq!(dt.timestamp(), 1736936173);
/// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2025-01-15 18:16:13");
/// ```
pub fn from_str(s: &str, fmt: &str) -> Result<DateTime<Local>>{
    let datetime = NaiveDateTime::parse_from_str(s, fmt)?;
    let tz = Local::now().timezone();
    // 将 NaiveDateTime 转换为具有时区的 DateTime
    let dt = tz.from_local_datetime(&datetime).single().unwrap();
    Ok(dt)
}

/// 通过时间字符串解析得到带时区的 `DateTime<Tz>` 结构体
///
/// Examples
///
/// ```
/// use lazytool::time;
///
/// let dt = time::from_str_with_timezone("2025-01-15 18:16:13", "%Y-%m-%d %H:%M:%S", "Asia/Shanghai").unwrap();
///
/// assert_eq!(dt.timestamp(), 1736936173);
/// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2025-01-15 18:16:13");
/// ```
pub fn from_str_with_timezone(s: &str, fmt: &str, timezone: &str) -> Result<DateTime<Tz>>{
    let datetime = NaiveDateTime::parse_from_str(s, fmt)?;
    // 获取时区
    let tz: Tz = timezone.parse()?;
    // 将 NaiveDateTime 转换为具有时区的 DateTime
    let dt = tz.from_local_datetime(&datetime).single().unwrap();
    Ok(dt)
}

/// 字符串转为时间戳
///
/// Examples
///
/// ```
/// use lazytool::time;
///
/// let dt = time::to_timestamp("2025-01-15 18:16:13", "%Y-%m-%d %H:%M:%S").unwrap();
///
/// assert_eq!(dt, 1736936173);
/// ```
pub fn to_timestamp(s: &str, fmt: &str) -> Result<i64>{
    let dt = from_str(s, fmt)?;
    Ok(dt.timestamp())
}
