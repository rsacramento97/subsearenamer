use fs2::available_space;
use sha2::{Digest, Sha256};
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
use thiserror::Error;

pub mod bridge_contract;
pub mod native_api;
pub mod job;

#[derive(Debug, Error)]
pub enum SafeCopyError {
    #[error("source does not exist or is not a file")]
    InvalidSource,
    #[error("source and destination must be different")]
    SamePath,
    #[error("destination must not be inside the source tree")]
    DestinationInsideSource,
    #[error("source must not be inside the destination tree")]
    SourceInsideDestination,
    #[error("destination already exists: {0}")]
    DestinationExists(String),
    #[error("insufficient disk space: required {required} bytes, available {available} bytes")]
    InsufficientSpace { required: u64, available: u64 },
    #[error("copy failed: {0}")]
    Copy(#[from] std::io::Error),
    #[error("integrity verification failed")]
    IntegrityMismatch,
}

#[derive(Debug, Clone)]
pub struct CopyReport { pub source: PathBuf, pub destination: PathBuf, pub bytes: u64, pub sha256: Option<String> }

fn sha256_file(path: &Path) -> Result<String, SafeCopyError> { let mut file=File::open(path)?; let mut hasher=Sha256::new(); let mut buffer=[0u8;1024*1024]; loop { let read=file.read(&mut buffer)?; if read==0 {break;} hasher.update(&buffer[..read]); } Ok(hex::encode(hasher.finalize())) }
fn canonical_parent(path:&Path)->Result<PathBuf,SafeCopyError>{Ok(fs::canonicalize(path.parent().unwrap_or_else(||Path::new(".")))?)}
fn normalized_destination(path:&Path)->Result<PathBuf,SafeCopyError>{Ok(canonical_parent(path)?.join(path.file_name().ok_or(SafeCopyError::InvalidSource)?))}
fn ensure_destination_is_outside_source(source:&Path,destination:&Path)->Result<(),SafeCopyError>{let source_root=fs::canonicalize(source.parent().ok_or(SafeCopyError::InvalidSource)?)?;let destination_parent=canonical_parent(destination)?;if destination_parent!=source_root&&destination_parent.starts_with(&source_root){return Err(SafeCopyError::DestinationInsideSource)}Ok(())}
fn unique_temp_path(parent:&Path,final_name:&str)->PathBuf{let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map(|d|d.as_nanos()).unwrap_or_default();parent.join(format!(".{final_name}.{stamp}-{}.subsea-partial",std::process::id()))}

pub fn copy_verify_rename(source:&Path,destination:&Path,verify_hash:bool)->Result<CopyReport,SafeCopyError>{
    if !source.is_file(){return Err(SafeCopyError::InvalidSource)}
    let source_canonical=fs::canonicalize(source)?; if let Some(parent)=destination.parent(){fs::create_dir_all(parent)?;}
    let destination_normalized=normalized_destination(destination)?; if source_canonical==destination_normalized{return Err(SafeCopyError::SamePath)}
    ensure_destination_is_outside_source(source,destination)?; if destination.exists(){return Err(SafeCopyError::DestinationExists(destination.display().to_string()))}
    let required=fs::metadata(source)?.len(); let parent=destination.parent().unwrap_or_else(||Path::new(".")); let available=available_space(parent)?; if available<required{return Err(SafeCopyError::InsufficientSpace{required,available})}
    let source_hash_before=if verify_hash{Some(sha256_file(source)?)}else{None}; let temp=unique_temp_path(parent,destination.file_name().unwrap().to_string_lossy().as_ref());
    let mut input=File::open(source)?; let mut output=OpenOptions::new().write(true).create_new(true).open(&temp)?;
    let copy_result=(||->Result<u64,SafeCopyError>{let mut buffer=[0u8;1024*1024];let mut copied=0u64;loop{let read=input.read(&mut buffer)?;if read==0{break;}output.write_all(&buffer[..read])?;copied+=read as u64;}output.sync_all()?;Ok(copied)})();
    let copied=match copy_result{Ok(bytes)=>bytes,Err(error)=>{drop(output);let _=fs::remove_file(&temp);return Err(error)}}; drop(output);
    if fs::metadata(&temp)?.len()!=required||copied!=required{let _=fs::remove_file(&temp);return Err(SafeCopyError::IntegrityMismatch)}
    let source_hash=if verify_hash{let after=sha256_file(source)?;let before=source_hash_before.as_ref().expect("hash enabled");let actual=sha256_file(&temp)?;if &after!=before||&actual!=before{let _=fs::remove_file(&temp);return Err(SafeCopyError::IntegrityMismatch)}Some(after)}else{None};
    if destination.exists(){let _=fs::remove_file(&temp);return Err(SafeCopyError::DestinationExists(destination.display().to_string()))}
    if let Err(error)=fs::rename(&temp,destination){let _=fs::remove_file(&temp);return Err(SafeCopyError::Copy(error))}
    Ok(CopyReport{source:source.to_path_buf(),destination:destination.to_path_buf(),bytes:copied,sha256:source_hash})
}

#[cfg(test)]
mod tests{use super::*;#[test]fn bridge_contract_serializes(){let request=crate::bridge_contract::RenameJobRequest{source_dir:"C:/src".into(),destination_dir:"D:/dst".into(),timezone:"UTC".into(),manual_offset_minutes:Some(-180),verify_hash:true};let value=serde_json::to_value(request).unwrap();assert_eq!(value["sourceDir"],"C:/src");assert_eq!(value["manualOffsetMinutes"],-180)}#[test]fn preview_status_serializes_stably(){assert_eq!(serde_json::to_string(&crate::bridge_contract::PreviewStatus::Ready).unwrap(),"\"ready\"");assert_eq!(serde_json::to_string(&crate::bridge_contract::PreviewStatus::Conflict).unwrap(),"\"conflict\"")}}
