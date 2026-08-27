//! Explicit timestamp/fuso primitives. No implicit machine-local timezone is used.

use chrono::{DateTime, FixedOffset, Utc};

pub fn parse_fixed_offset(minutes: i32) -> Result<FixedOffset, String> {
    if !(-14 * 60..=14 * 60).contains(&minutes) { return Err("timezone offset must be between -14:00 and +14:00".into()); }
    FixedOffset::east_opt(minutes * 60).ok_or_else(|| "invalid timezone offset".into())
}

pub fn utc_to_offset(utc: DateTime<Utc>, offset_minutes: i32) -> Result<DateTime<FixedOffset>, String> {
    Ok(utc.with_timezone(&parse_fixed_offset(offset_minutes)?))
}

pub fn format_compact(dt: DateTime<FixedOffset>) -> String { dt.format("%Y%m%d_%H%M%S").to_string() }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn converts_without_using_machine_timezone() {
        let utc = DateTime::parse_from_rfc3339("2026-08-27T18:30:45Z").unwrap().with_timezone(&Utc);
        assert_eq!(format_compact(utc_to_offset(utc, -180).unwrap()), "20260827_153045");
    }
    #[test] fn rejects_out_of_range_offset() { assert!(parse_fixed_offset(841).is_err()); }
    #[test] fn accepts_both_signs() { assert!(parse_fixed_offset(-180).is_ok()); assert!(parse_fixed_offset(330).is_ok()); }
}
