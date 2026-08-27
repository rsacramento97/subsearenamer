//! Conservative execution engine. It copies to a new destination, verifies it,
//! and only then considers the operation complete. Originals are never deleted.

use crate::{copy_verify_rename, SafeCopyError};
use std::{path::Path, sync::{Arc, atomic::{AtomicBool, Ordering}}};

#[derive(Debug, Clone)]
pub struct ExecutionItem { pub source: String, pub destination: String }

#[derive(Debug, Clone, Default)]
pub struct ExecutionReport { pub completed: u64, pub failed: u64, pub cancelled: bool, pub errors: Vec<String> }

pub fn execute(items: &[ExecutionItem], verify_hash: bool, cancel: &Arc<AtomicBool>) -> ExecutionReport {
    let mut report = ExecutionReport::default();
    for item in items {
        if cancel.load(Ordering::Relaxed) { report.cancelled = true; break; }
        match copy_verify_rename(Path::new(&item.source), Path::new(&item.destination), verify_hash) {
            Ok(_) => report.completed += 1,
            Err(error) => { report.failed += 1; report.errors.push(format!("{}: {}", item.source, format_error(error))); }
        }
    }
    report
}

fn format_error(error: SafeCopyError) -> String { error.to_string() }

#[cfg(test)]
mod tests { use super::*; #[test] fn cancellation_before_start_does_not_touch_files(){let c=Arc::new(AtomicBool::new(true));let r=execute(&[ExecutionItem{source:"missing.mp4".into(),destination:"out.mp4".into()}],true,&c);assert!(r.cancelled);assert_eq!(r.completed,0);assert_eq!(r.failed,0);} }
