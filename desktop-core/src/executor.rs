//! Conservative batch executor. Source files are never renamed in place.
//! Each destination is written to a unique partial file, flushed, verified,
//! then atomically moved to its final name. Existing destinations are never overwritten.

use crate::{copy_verify_rename, filename_plan::{planned_filename, VideoNameInput}};
use std::{fs, path::{Path, PathBuf}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionItem { pub source: PathBuf, pub destination: PathBuf }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReport { pub completed: usize, pub failed: usize, pub cancelled: bool, pub errors: Vec<String> }

pub fn build_execution_plan(items: &[(VideoNameInput, i32)], destination_dir: &Path) -> Result<Vec<ExecutionItem>, String> {
    let mut plan = Vec::with_capacity(items.len());
    for (input, offset) in items {
        let source = PathBuf::from(&input.path);
        let filename = planned_filename(input, *offset)?;
        plan.push(ExecutionItem { source, destination: destination_dir.join(filename) });
    }
    plan.sort_by(|a,b| a.source.to_string_lossy().cmp(&b.source.to_string_lossy()));
    for pair in plan.windows(2) {
        if pair[0].destination == pair[1].destination { return Err(format!("duplicate destination: {}", pair[0].destination.display())); }
    }
    Ok(plan)
}

pub fn execute_plan(plan: &[ExecutionItem], verify_hash: bool) -> ExecutionReport {
    let mut report = ExecutionReport { completed: 0, failed: 0, cancelled: false, errors: Vec::new() };
    for item in plan {
        if item.destination.exists() {
            report.failed += 1;
            report.errors.push(format!("destination already exists: {}", item.destination.display()));
            continue;
        }
        match copy_verify_rename(&item.source, &item.destination, verify_hash) {
            Ok(_) => report.completed += 1,
            Err(error) => {
                report.failed += 1;
                report.errors.push(format!("{}: {}", item.source.display(), error));
            }
        }
    }
    report
}

pub fn cleanup_empty_destination_dirs(_root: &Path) -> Result<(), String> { Ok(()) }

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    #[test]
    fn plan_rejects_duplicate_destinations() {
        let t=DateTime::parse_from_rfc3339("2026-08-27T18:30:45Z").unwrap().with_timezone(&Utc);
        let a=VideoNameInput{path:"a.mp4".into(),captured_utc:t,sequence:1};
        let b=VideoNameInput{path:"b.mov".into(),captured_utc:t,sequence:1};
        assert!(build_execution_plan(&[(a,0),(b,0)],Path::new("dst")).is_err());
    }
    #[test]
    fn empty_plan_is_safe() { let report=execute_plan(&[],true); assert_eq!(report.completed,0); assert_eq!(report.failed,0); }
    #[test]
    fn destination_path_is_derived_without_touching_source() {
        let t=DateTime::parse_from_rfc3339("2026-08-27T18:30:45Z").unwrap().with_timezone(&Utc);
        let input=VideoNameInput{path:"a.mp4".into(),captured_utc:t,sequence:1};
        let plan=build_execution_plan(&[(input,0)],Path::new("dst")).unwrap();
        assert_eq!(plan[0].destination,Path::new("dst/20260827_183045_0001.mp4"));
        assert!(fs::metadata("a.mp4").is_err());
    }
}
