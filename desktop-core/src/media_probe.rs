use crate::metadata::{parse_timestamp, VideoMetadata};
use std::{path::{Path, PathBuf}, process::Command};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeOutput { pub captured_utc: chrono::DateTime<chrono::Utc>, pub source: String }
pub trait MediaProbe { fn probe(&self, path: &Path) -> Result<ProbeOutput, String>; }
#[derive(Debug, Clone)] pub struct Ffprobe { pub executable: PathBuf }
impl Ffprobe { pub fn new(executable: impl Into<PathBuf>) -> Self { Self { executable: executable.into() } } }
impl MediaProbe for Ffprobe {
 fn probe(&self, path:&Path)->Result<ProbeOutput,String>{
  let output=Command::new(&self.executable).args(["-v","error","-show_entries","format_tags=creation_time:stream_tags=creation_time","-of","default=nw=1:nk=1"]).arg(path).output().map_err(|e|format!("failed to start media probe: {e}"))?;
  if !output.status.success(){return Err("media probe failed".into());}
  let value=String::from_utf8_lossy(&output.stdout).lines().map(str::trim).find(|v|!v.is_empty()).ok_or("capture timestamp not found")?;
  Ok(ProbeOutput{captured_utc:parse_timestamp(value)?,source:"ffprobe:creation_time".into()})
 }
}
impl From<ProbeOutput> for VideoMetadata { fn from(v:ProbeOutput)->Self{Self{captured_utc:v.captured_utc,source:v.source}} }
