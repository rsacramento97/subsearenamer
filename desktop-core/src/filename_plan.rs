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

pub fn planned_filename_with_index(input: &VideoNameInput, offset_minutes: i32, collision_index: usize) -> Result<String, String> {
    let base = planned_filename(input, offset_minutes)?;
    if collision_index == 0 { return Ok(base); }
    let path = Path::new(&base);
    let stem = path.file_stem().and_then(|s| s.to_str()).ok_or("invalid planned filename")?;
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let candidate = format!("{}_{}", stem, collision_index + 1);
    Ok(if ext.is_empty() { candidate } else { format!("{}.{}", candidate, ext) })
}

#[cfg(test)]
mod tests { use super::*; #[test]fn builds_expected_name(){let i=VideoNameInput{path:"ROV.MP4".into(),captured_utc:DateTime::parse_from_rfc3339("2026-08-27T18:30:45Z").unwrap().with_timezone(&Utc),sequence:7};assert_eq!(planned_filename(&i,-180).unwrap(),"20260827_153045_0007.MP4");} }
