//! Strict metadata boundary. Timezone-less timestamps are rejected.
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoMetadata { pub captured_utc: DateTime<Utc>, pub source: String }

pub fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, String> {
    let parsed = DateTime::parse_from_rfc3339(value.trim()).map_err(|_| "metadata timestamp must be RFC3339 with an explicit offset".to_string())?;
    Ok(parsed.with_timezone(&Utc))
}

#[cfg(test)]
mod tests { use super::*; #[test] fn accepts_utc(){assert!(parse_timestamp("2026-08-27T18:30:45Z").is_ok())} #[test] fn rejects_local_time(){assert!(parse_timestamp("2026-08-27 18:30:45").is_err())} }
