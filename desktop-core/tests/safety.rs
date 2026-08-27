use std::{fs, path::PathBuf};
use subsea_renamer_core::{copy_verify_rename, SafeCopyError};

fn test_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("subsea-renamer-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn copies_and_verifies_without_touching_source() {
    let root = test_root("copy");
    let source_dir = root.join("SOURCE");
    let destination_dir = root.join("RENAMED_VIDEOS");
    fs::create_dir_all(&source_dir).unwrap();
    let source = source_dir.join("original.MP4");
    let destination = destination_dir.join("20260827_123422_ROV01_CAM01.MP4");
    let bytes = b"test video bytes - source must remain identical";
    fs::write(&source, bytes).unwrap();
    let before = fs::read(&source).unwrap();

    let report = copy_verify_rename(&source, &destination, true).unwrap();
    assert_eq!(report.bytes, bytes.len() as u64);
    assert!(report.sha256.is_some());
    assert_eq!(fs::read(&source).unwrap(), before);
    assert_eq!(fs::read(&destination).unwrap(), bytes);

    let overwrite = copy_verify_rename(&source, &destination, true);
    assert!(matches!(overwrite, Err(SafeCopyError::DestinationExists(_))));
    assert_eq!(fs::read(&source).unwrap(), before);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn rejects_destination_inside_source_tree() {
    let root = test_root("overlap");
    let source_dir = root.join("SOURCE");
    fs::create_dir_all(&source_dir).unwrap();
    let source = source_dir.join("original.MP4");
    let destination = source_dir.join("RENAMED_VIDEOS").join("copy.MP4");
    fs::write(&source, b"immutable source").unwrap();

    let result = copy_verify_rename(&source, &destination, true);
    assert!(matches!(result, Err(SafeCopyError::DestinationInsideSource)));
    assert_eq!(fs::read(&source).unwrap(), b"immutable source");
    assert!(!destination.exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn source_and_destination_are_never_the_same_path() {
    let root = test_root("same-path");
    let source = root.join("SOURCE").join("example.mp4");
    fs::create_dir_all(source.parent().unwrap()).unwrap();
    fs::write(&source, b"source").unwrap();
    let result = copy_verify_rename(&source, &source, false);
    assert!(matches!(result, Err(SafeCopyError::SamePath)));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn failed_destination_is_not_left_as_partial_file() {
    let root = test_root("partial");
    let source_dir = root.join("SOURCE");
    let destination_dir = root.join("RENAMED_VIDEOS");
    fs::create_dir_all(&source_dir).unwrap();
    fs::create_dir_all(&destination_dir).unwrap();
    let source = source_dir.join("original.mp4");
    let destination = destination_dir.join("copy.mp4");
    fs::write(&source, b"safe bytes").unwrap();

    let result = copy_verify_rename(&source, &destination, true);
    assert!(result.is_ok());
    let partials = fs::read_dir(&destination_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().contains("subsea-partial"))
        .count();
    assert_eq!(partials, 0);
    assert_eq!(fs::read(&source).unwrap(), b"safe bytes");
    let _ = fs::remove_dir_all(&root);
}
