use chrono::{DateTime, FixedOffset, Utc};

pub fn utc_to_offset(value: DateTime<Utc>, offset_minutes: i32) -> Result<DateTime<FixedOffset>, String> {
    if !(-840..=840).contains(&offset_minutes) { return Err("timezone offset must be between -14:00 and +14:00".into()); }
    let offset = FixedOffset::east_opt(offset_minutes * 60).ok_or("invalid timezone offset")?;
    Ok(value.with_timezone(&offset))
}

pub fn format_compact(value: DateTime<FixedOffset>) -> String { value.format("%Y%m%d_%H%M%S").to_string() }

#[cfg(test)]
mod tests { use super::*; #[test] fn converts_negative_three(){let u=DateTime::parse_from_rfc3339("2026-08-27T18:30:45Z").unwrap().with_timezone(&Utc);assert_eq!(format_compact(utc_to_offset(u,-180).unwrap()),"20260827_153045");} #[test]fn rejects_out_of_range(){let u=Utc::now();assert!(utc_to_offset(u,841).is_err());} }
