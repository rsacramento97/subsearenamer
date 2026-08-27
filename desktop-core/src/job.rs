//! Filesystem discovery for preview. Discovery itself never renames or writes video files.

use std::{path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use walkdir::WalkDir;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "m4v", "mkv", "avi", "mts", "m2ts", "ts", "webm"];

pub fn is_video_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).map(|e| VIDEO_EXTENSIONS.iter().any(|x| e.eq_ignore_ascii_case(x))).unwrap_or(false)
}

pub fn discover_videos(root: &Path, cancel: Option<&Arc<AtomicBool>>) -> Result<Vec<PathBuf>, String> {
    if !root.is_dir() { return Err(format!("source directory does not exist or is not a directory: {}", root.display())); }
    let mut files = Vec::new();
    for entry in WalkDir::new(root).follow_links(false).into_iter().filter_map(Result::ok) {
        if cancel.map(|c| c.load(Ordering::Relaxed)).unwrap_or(false) { break; }
        let path = entry.path();
        if entry.file_type().is_file() && is_video_file(path) { files.push(path.to_path_buf()); }
    }
    files.sort_by(|a,b| a.to_string_lossy().cmp(&b.to_string_lossy()));
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn extension_check_is_case_insensitive(){assert!(is_video_file(Path::new("x.MP4")));assert!(!is_video_file(Path::new("x.txt")));}
}
