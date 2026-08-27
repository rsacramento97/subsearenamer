use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameParts { pub extension: String }

pub fn split_video_name(path: &Path) -> Result<NameParts, String> {
    let extension = path.extension().and_then(|e| e.to_str()).ok_or("video has no valid extension")?;
    if extension.is_empty() { return Err("video has empty extension".into()); }
    Ok(NameParts { extension: extension.to_string() })
}

pub fn compose_name(parts: &[&str], extension: &str) -> Result<String, String> {
    if parts.is_empty() || parts.iter().any(|p| p.is_empty()) { return Err("filename components cannot be empty".into()); }
    if extension.is_empty() { return Err("extension is required".into()); }
    let stem = parts.join("_");
    if stem.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) { return Err("filename contains an invalid Windows character".into()); }
    Ok(format!("{stem}.{extension}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn preserves_extension(){assert_eq!(split_video_name(Path::new("a.MP4")).unwrap().extension,"MP4");}
    #[test] fn rejects_windows_separators(){assert!(compose_name(&["a/b"],"mp4").is_err());}
}
