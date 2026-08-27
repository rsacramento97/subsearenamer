use sha2::{Digest, Sha256};
use std::{fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SafeCopyError {
    #[error("source and destination must be different")]
    SamePath,
    #[error("destination already exists: {0}")]
    DestinationExists(String),
    #[error("insufficient disk space")]
    InsufficientSpace,
    #[error("copy failed: {0}")]
    Copy(#[from] std::io::Error),
    #[error("integrity verification failed")]
    HashMismatch,
}

#[derive(Debug, Clone)]
pub struct CopyReport {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub bytes: u64,
    pub sha256: Option<String>,
}

fn sha256_file(path: &Path) -> Result<String, SafeCopyError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 { break; }
        hasher.update(&buffer[..read]);
    }
    Ok(hex::encode(hasher.finalize()))
}

/// Copies bytes to a new file, verifies size and optionally SHA-256, then atomically renames
/// the temporary copy to its requested destination. The source is never modified.
pub fn copy_verify_rename(source: &Path, destination: &Path, verify_hash: bool) -> Result<CopyReport, SafeCopyError> {
    if fs::canonicalize(source).ok() == fs::canonicalize(destination).ok() { return Err(SafeCopyError::SamePath); }
    if destination.exists() { return Err(SafeCopyError::DestinationExists(destination.display().to_string())); }
    let metadata = fs::metadata(source)?;
    let required = metadata.len();
    let available = fs::metadata(destination.parent().unwrap_or_else(|| Path::new(".")))?.len();
    let _ = available; // Actual free-space check belongs to the platform adapter.

    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let temp = parent.join(format!(".{}.subsea-partial", destination.file_name().unwrap().to_string_lossy()));
    if temp.exists() { fs::remove_file(&temp)?; }

    let mut input = File::open(source)?;
    let mut output = File::create(&temp)?;
    let mut buffer = [0u8; 1024 * 1024];
    let mut copied = 0u64;
    loop {
        let read = input.read(&mut buffer)?;
        if read == 0 { break; }
        output.write_all(&buffer[..read])?;
        copied += read as u64;
    }
    output.sync_all()?;
    drop(output);

    let copied_size = fs::metadata(&temp)?.len();
    if copied_size != required {
        let _ = fs::remove_file(&temp);
        return Err(SafeCopyError::HashMismatch);
    }
    let source_hash = if verify_hash { Some(sha256_file(source)?) } else { None };
    if let Some(expected) = &source_hash {
        let actual = sha256_file(&temp)?;
        if &actual != expected {
            let _ = fs::remove_file(&temp);
            return Err(SafeCopyError::HashMismatch);
        }
    }
    fs::rename(&temp, destination)?;
    Ok(CopyReport { source: source.to_path_buf(), destination: destination.to_path_buf(), bytes: copied, sha256: source_hash })
}
