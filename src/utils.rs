use std::num::ParseIntError;

use std::time::Duration;

pub fn parse_duration(s: &str) -> Result<Duration, String> {
    let len = s.len();
    if len < 2 {
        return Err("Invalid duration format".to_string());
    }
    let (num_str, unit) = s.split_at(len - 1);
    let num: u64 = num_str.parse().map_err(|e: ParseIntError| e.to_string())?;

    match unit {
        "s" => Ok(Duration::from_secs(num)),
        "m" => Ok(Duration::from_secs(num * 60)),
        "h" => Ok(Duration::from_secs(num * 60 * 60)),
        "d" => Ok(Duration::from_secs(num * 60 * 60 * 24)),
        _ => Err("Invalid duration unit".to_string()),
    }
}
