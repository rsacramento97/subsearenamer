use std::{fs, path::PathBuf};
use subsea_renamer_core::{copy_verify_rename, SafeCopyError};

#[test]
fn copies_and_verifies_without_touching_source() {
    let root = std::env::temp_dir().join(format!("subsea-renamer-test-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let source = root.join("original.MP4");
    let destination = root.join("RENAMED_VIDEOS").join("20260827_123422_ROV01_CAM01.MP4");
    let bytes = b"test video bytes - source must remain identical";
    fs::write(&source, bytes).unwrap();
    let before = fs::read(&source).unwrap();

    let report = copy_verify_rename(&source, &destination, true).unwrap();
    assert_eq!(report.bytes, bytes.len() as u64);
    assert_eq!(fs::read(&source).unwrap(), before);
    assert_eq!(fs::read(&destination).unwrap(), bytes);

    let overwrite = copy_verify_rename(&source, &destination, true);
    assert!(matches!(overwrite, Err(SafeCopyError::DestinationExists(_))));
    assert_eq!(fs::read(&source).unwrap(), before);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn source_and_destination_are_never_the_same_path() {
    let path = PathBuf::from("example.mp4");
    let result = copy_verify_rename(&path, &path, false);
    assert!(matches!(result, Err(SafeCopyError::SamePath)));
}
