//! Deterministic video discovery and job planning.
//! This module only plans work; it never mutates source files.

use crate::bridge_contract::{PreviewItem, PreviewStatus, RenameJobRequest};
use crate::native_api::validate_request;
use std::{fs, path::{Path, PathBuf}};
use walkdir::WalkDir;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "m4v", "avi", "mkv", "mts", "m2ts", "ts", "wmv"];

fn is_video(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).map(|e| VIDEO_EXTENSIONS.iter().any(|x| e.eq_ignore_ascii_case(x))).unwrap_or(false)
}

pub fn discover_videos(source_dir: &Path) -> Result<Vec<PathBuf>, String> {
    if !source_dir.is_dir() { return Err("sourceDir must be an existing directory".into()); }
    let mut files = Vec::new();
    for entry in WalkDir::new(source_dir).follow_links(false) {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if entry.file_type().is_file() && is_video(path) { files.push(path.to_path_buf()); }
    }
    files.sort_by(|a,b| a.to_string_lossy().cmp(&b.to_string_lossy()));
    Ok(files)
}

pub fn plan_preview(request: &RenameJobRequest) -> Result<Vec<PreviewItem>, String> {
    validate_request(request)?;
    let source_dir = Path::new(&request.source_dir);
    let destination_dir = Path::new(&request.destination_dir);
    let videos = discover_videos(source_dir)?;
    let mut result = Vec::with_capacity(videos.len());
    for source in videos {
        let relative = source.strip_prefix(source_dir).map_err(|e| e.to_string())?;
        let destination = destination_dir.join(relative);
        let (status, reason) = if destination.exists() {
            (PreviewStatus::Conflict, Some("destination already exists; source will not be overwritten".into()))
        } else {
            (PreviewStatus::Ready, None)
        };
        result.push(PreviewItem { source: source.to_string_lossy().into_owned(), destination: destination.to_string_lossy().into_owned(), status, reason });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::{self, File}, io::Write};

    #[test]
    fn discovers_video_extensions_case_insensitively_and_sorts() {
        let dir = std::env::temp_dir().join(format!("subsea-job-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir); fs::create_dir_all(&dir).unwrap();
        for name in ["b.MP4", "a.mov", "ignore.txt"] { let mut f=File::create(dir.join(name)).unwrap(); writeln!(f,"x").unwrap(); }
        let found = discover_videos(&dir).unwrap();
        assert_eq!(found.len(), 2); assert!(found[0].file_name().unwrap() < found[1].file_name().unwrap());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn preview_marks_existing_destination_as_conflict() {
        let root = std::env::temp_dir().join(format!("subsea-preview-{}", std::process::id()));
        let src=root.join("src"); let dst=root.join("dst"); let _=fs::remove_dir_all(&root); fs::create_dir_all(&src).unwrap(); fs::create_dir_all(&dst).unwrap();
        File::create(src.join("video.mp4")).unwrap(); File::create(dst.join("video.mp4")).unwrap();
        let request=RenameJobRequest{source_dir:src.to_string_lossy().into(),destination_dir:dst.to_string_lossy().into(),timezone:"UTC".into(),manual_offset_minutes:None,verify_hash:true};
        let preview=plan_preview(&request).unwrap(); assert_eq!(preview.len(),1); assert!(matches!(preview[0].status,PreviewStatus::Conflict));
        fs::remove_dir_all(&root).unwrap();
    }
}
