use penumbra_registry::parser::reset_registry_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempdir::TempDir;

fn create_file_in_dir(dir: &Path, file_name: &str) {
    let file_path = dir.join(file_name);
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "Test content").unwrap();
}

fn create_subdir_in_dir(dir: &Path, subdir_name: &str) {
    let subdir_path = dir.join(subdir_name);
    fs::create_dir_all(subdir_path).unwrap();
}

#[test]
fn test_reset_non_existent_directory() {
    let temp_dir = TempDir::new("").unwrap();
    let non_existent_dir = temp_dir.path().join("nonexistent");

    reset_registry_dir(non_existent_dir.to_str().unwrap()).unwrap();
    assert!(non_existent_dir.exists());
    assert!(non_existent_dir.join("chains").exists());
}

#[test]
fn test_reset_existing_directory_with_files() {
    let temp_dir = TempDir::new("").unwrap();
    create_file_in_dir(temp_dir.path(), "testfile.txt");

    reset_registry_dir(temp_dir.path().to_str().unwrap()).unwrap();
    assert!(temp_dir.path().join("chains").exists());
    assert!(!temp_dir.path().join("testfile.txt").exists());
}

#[test]
fn test_reset_existing_directory_with_subdirectories() {
    let temp_dir = TempDir::new("").unwrap();
    create_subdir_in_dir(temp_dir.path(), "subdir");

    reset_registry_dir(temp_dir.path().to_str().unwrap()).unwrap();
    assert!(temp_dir.path().join("chains").exists());
    assert!(!temp_dir.path().join("subdir").exists());
}
