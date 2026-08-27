//! Stable request/response contract for the future Tauri adapter.
//! The core remains UI-agnostic: this module contains no Tauri or browser code.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameJobRequest {
    pub source_dir: String,
    pub destination_dir: String,
    pub timezone: String,
    pub manual_offset_minutes: Option<i32>,
    pub verify_hash: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewItem {
    pub source: String,
    pub destination: String,
    pub status: PreviewStatus,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewStatus {
    Ready,
    Conflict,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameJobResult {
    pub completed: u64,
    pub failed: u64,
    pub cancelled: bool,
}
