//! UI-facing native API boundary. This module deliberately does not perform
//! filesystem mutation itself; adapters must call the validated core engine.

use crate::bridge_contract::{PreviewItem, PreviewStatus, RenameJobRequest, RenameJobResult};
use std::path::Path;

pub fn validate_request(request: &RenameJobRequest) -> Result<(), String> {
    if request.source_dir.trim().is_empty() { return Err("sourceDir is required".into()); }
    if request.destination_dir.trim().is_empty() { return Err("destinationDir is required".into()); }
    if request.timezone.trim().is_empty() { return Err("timezone is required".into()); }
    if let Some(offset) = request.manual_offset_minutes {
        if !(-14 * 60..=14 * 60).contains(&offset) { return Err("manualOffsetMinutes must be between -840 and 840".into()); }
    }
    Ok(())
}

pub fn preview_request(request: &RenameJobRequest) -> Result<Vec<PreviewItem>, String> {
    validate_request(request)?;
    let source = Path::new(&request.source_dir);
    let destination = Path::new(&request.destination_dir);
    if !source.exists() { return Ok(vec![PreviewItem { source: request.source_dir.clone(), destination: request.destination_dir.clone(), status: PreviewStatus::Invalid, reason: Some("source directory does not exist".into()) }]); }
    if !destination.exists() { return Ok(vec![PreviewItem { source: request.source_dir.clone(), destination: request.destination_dir.clone(), status: PreviewStatus::Ready, reason: Some("destination directory will be created during execution".into()) }]); }
    Ok(vec![PreviewItem { source: request.source_dir.clone(), destination: request.destination_dir.clone(), status: PreviewStatus::Ready, reason: None }])
}

pub fn cancelled_result() -> RenameJobResult {
    RenameJobResult { completed: 0, failed: 0, cancelled: true }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> RenameJobRequest { RenameJobRequest { source_dir: "src".into(), destination_dir: "dst".into(), timezone: "UTC".into(), manual_offset_minutes: None, verify_hash: true } }

    #[test]
    fn rejects_empty_paths() { let mut r = request(); r.source_dir.clear(); assert!(validate_request(&r).is_err()); }
    #[test]
    fn rejects_invalid_offset() { let mut r = request(); r.manual_offset_minutes = Some(841); assert!(validate_request(&r).is_err()); }
    #[test]
    fn accepts_valid_offset() { let mut r = request(); r.manual_offset_minutes = Some(-180); assert!(validate_request(&r).is_ok()); }
    #[test]
    fn cancellation_is_explicit() { assert!(cancelled_result().cancelled); }
}
