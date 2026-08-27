//! Metadata adapter boundary. The core does not trust filename timestamps.
//! A future media backend (e.g. ffprobe/MediaInfo) can implement this trait.

use chrono::{DateTime, Utc};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoMetadata { pub captured_utc: DateTime<Utc>, pub source: String }

pub trait MetadataReader { fn read(&self, path: &Path) -> Result<VideoMetadata, String>; }

/// Strict parser for metadata values returned by an external media tool.
/// Only RFC3339 timestamps are accepted; ambiguous local timestamps are rejected.
pub fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, String> {
    let parsed = DateTime::parse_from_rfc3339(value).map_err(|_| "metadata timestamp must be RFC3339 with an explicit offset".to_string())?;
    Ok(parsed.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn accepts_explicit_offset(){assert_eq!(parse_timestamp("2026-08-27T18:30:45Z").unwrap().to_rfc3339(),"2026-08-27T18:30:45+00:00");}
    #[test] fn rejects_ambiguous_local_time(){assert!(parse_timestamp("2026-08-27 18:30:45").is_err());}
}
