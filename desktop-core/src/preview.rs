//! Pure preview orchestration. No destination file is created here.

use crate::{bridge_contract::{PreviewItem, PreviewStatus, RenameJobRequest}, job::discover_videos, media_probe::{Ffprobe, MediaProbe}, filename_plan::{planned_filename, VideoNameInput}};
use std::{collections::HashSet, path::Path};

pub fn preview(request: &RenameJobRequest, ffprobe: &Ffprobe) -> Result<Vec<PreviewItem>, String> {
    if Path::new(&request.source_dir) == Path::new(&request.destination_dir) { return Err("sourceDir and destinationDir must differ".into()); }
    let files = discover_videos(Path::new(&request.source_dir), None)?;
    let mut destinations = HashSet::new();
    let mut result = Vec::with_capacity(files.len());
    for (sequence, source) in files.into_iter().enumerate() {
        let metadata = match ffprobe.probe(&source) {
            Ok(v) => v,
            Err(e) => { result.push(PreviewItem{source:source.display().to_string(),destination:String::new(),status:PreviewStatus::Invalid,reason:Some(e)}); continue; }
        };
        let input = VideoNameInput{path:source.display().to_string(),captured_utc:metadata.captured_utc,sequence:(sequence+1) as u64};
        let filename = match planned_filename(&input, request.manual_offset_minutes.unwrap_or(0)) {
            Ok(v) => v,
            Err(e) => { result.push(PreviewItem{source:source.display().to_string(),destination:String::new(),status:PreviewStatus::Invalid,reason:Some(e)}); continue; }
        };
        let destination = Path::new(&request.destination_dir).join(filename);
        let destination_string = destination.display().to_string();
        let conflict = destination.exists() || !destinations.insert(destination_string.clone());
        result.push(PreviewItem{source:source.display().to_string(),destination:destination_string,status:if conflict{PreviewStatus::Conflict}else{PreviewStatus::Ready},reason:if conflict{Some("destination already exists or is duplicated in this batch".into())}else{None}});
    }
    Ok(result)
}

#[cfg(test)]
mod tests { use super::*; #[test] fn empty_source_is_valid_preview(){let dir=tempfile::tempdir().unwrap();let dst=tempfile::tempdir().unwrap();let req=RenameJobRequest{source_dir:dir.path().display().to_string(),destination_dir:dst.path().display().to_string(),timezone:"UTC".into(),manual_offset_minutes:Some(0),verify_hash:true};let r=preview(&req,&Ffprobe::new("ffprobe" )).unwrap();assert!(r.is_empty());} }
