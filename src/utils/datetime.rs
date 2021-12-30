use std::time::SystemTime;
use chrono::{prelude::*, NaiveDateTime};

/// get current time stamp
pub fn timestamp() -> u64 { 
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) =>  { n.as_secs() },
        Err(_) =>  { 0 }
    }
}

/// 要求输入: 2019-11-11 10:10:10
pub fn from_str(datetime_str: &str) -> DateTime<Local> { 
    let date_time_arr = datetime_str.split(" ").collect::<Vec<&str>>();
    let y_m_d = date_time_arr[0].split("-").collect::<Vec<&str>>();
    let h_m_s = date_time_arr[1].split(":").collect::<Vec<&str>>();
    let year = if let Ok(v) = y_m_d[0].parse::<i32>() { v } else { 1970 };
    let month = if let Ok(v) = y_m_d[1].parse::<u32>() { v } else { 1 } ;
    let day = if let Ok(v) = y_m_d[2].parse::<u32>() { v } else { 1 } ;
    let hour = if let Ok(v) = h_m_s[0].parse::<u32>() { v } else { 1 };
    let minute = if let Ok(v) = h_m_s[1].parse::<u32>() { v } else { 1 };
    let second = if let Ok(v) = h_m_s[2].parse::<u32>() { v } else { 1 };
    Local.ymd(year, month, day).and_hms(hour, minute, second)
}

/// 当前的时间字符串
pub fn to_string() -> String { 
    let local: DateTime<Local> = Local::now();
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 格式化时间
pub fn format(format_str: &str) -> String { 
    let local: DateTime<Local> = Local::now();
    local.format(format_str).to_string()
}

/// 得到当前的日期
pub fn now() -> DateTime<Local> { 
    Local::now()
}

/// 得到时分秒
pub fn time() -> (u32, u32, u32) { 
    let now = now();
    (now.hour(), now.minute(), now.second())
}

/// 得到年月日
pub fn date() -> (u32, u32, u32) { 
    let now = now();
    (now.year() as u32, now.month() as u32, now.day() as u32)
}

/// 将时间戳转化为日期时间格式
pub fn datetime(timestamp: i64) -> String { 
    let dt = NaiveDateTime::from_timestamp(timestamp, 0);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
