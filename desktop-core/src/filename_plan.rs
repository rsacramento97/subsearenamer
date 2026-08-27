//! Pure filename planning: no filesystem writes and no implicit local timezone.

use crate::{naming::{compose_name, split_video_name}, timezone::{format_compact, utc_to_offset}};
use chrono::{DateTime, Utc};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoNameInput { pub path: String, pub captured_utc: DateTime<Utc>, pub sequence: u64 }

pub fn planned_filename(input: &VideoNameInput, offset_minutes: i32) -> Result<String, String> {
    let parts = split_video_name(Path::new(&input.path))?;
    let local = utc_to_offset(input.captured_utc, offset_minutes)?;
    compose_name(&[&format_compact(local), &format!("{:04}", input.sequence)], &parts.extension)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timestamp_is_shifted_before_filename_is_built() {
        let input=VideoNameInput{path:"ROV.MP4".into(),captured_utc:DateTime::parse_from_rfc3339("2026-08-27T18:30:45Z").unwrap().with_timezone(&Utc),sequence:7};
        assert_eq!(planned_filename(&input,-180).unwrap(),"20260827_153045_0007.MP4");
    }
    #[test]
    fn extension_is_preserved() {
        let input=VideoNameInput{path:"clip.mov".into(),captured_utc:DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap().with_timezone(&Utc),sequence:1};
        assert!(planned_filename(&input,0).unwrap().ends_with(".mov"));
    }
}
