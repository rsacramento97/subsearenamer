//! Deterministic, filesystem-safe naming primitives.
//! The exact project filename template can be applied by the adapter once
//! metadata parsing is available; this module deliberately never touches files.

use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameParts {
    pub original_stem: String,
    pub extension: String,
}

pub fn split_video_name(path: &Path) -> Result<NameParts, String> {
    let file_name = path.file_name().and_then(|v| v.to_str()).ok_or("invalid UTF-8 filename")?;
    let stem = path.file_stem().and_then(|v| v.to_str()).ok_or("invalid filename stem")?;
    let extension = path.extension().and_then(|v| v.to_str()).unwrap_or("");
    if stem.is_empty() { return Err("filename stem is empty".into()); }
    Ok(NameParts { original_stem: file_name[..file_name.len() - extension.len() - if extension.is_empty() { 0 } else { 1 }].to_string(), extension: extension.to_string() })
}

pub fn sanitize_component(value: &str) -> String {
    value.chars().map(|c| match c { '<'|'>'|':'|'"'|'/'|'\\'|'|'|'?'|'*' => '_', c if c.is_control() => '_', c => c }).collect::<String>().trim().trim_end_matches('.').to_string()
}

pub fn compose_name(parts: &[&str], extension: &str) -> Result<String, String> {
    let clean: Vec<String> = parts.iter().map(|p| sanitize_component(p)).filter(|p| !p.is_empty()).collect();
    if clean.is_empty() { return Err("filename would be empty".into()); }
    let ext = sanitize_component(extension).trim_start_matches('.').to_string();
    Ok(if ext.is_empty() { clean.join("_") } else { format!("{}.{}", clean.join("_"), ext) })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn sanitizes_windows_reserved_characters() { assert_eq!(sanitize_component("A:B/C\\D?"), "A_B_C_D_"); }
    #[test] fn composes_deterministically() { assert_eq!(compose_name(&["2026-08-27", "ROV", "001"], "MP4").unwrap(), "2026-08-27_ROV_001.MP4"); }
}
