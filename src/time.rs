use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳，单位秒
///
/// Examples
///
/// ```
/// use lazytool::current_timestamp;
///
/// let ts = current_timestamp();
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

