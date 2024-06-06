use penumbra_registry::parser::{copy_globals, Globals, Rpc};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempdir::TempDir;

fn create_file_with_content(dir: &Path, file_name: &str, content: &str) {
    let file_path = dir.join(file_name);
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "{}", content).unwrap();
}

#[test]
fn test_successful_copy() {
    let temp_input_dir = TempDir::new("").unwrap();
    let temp_output_dir = TempDir::new("").unwrap();
    let globals = Globals {
        rpcs: vec![Rpc {
            name: "cybernetics".to_string(),
            url: "http://api.zone".to_string(),
            images: vec![],
        }],
        frontends: vec!["http://frontend.zone".to_string()],
    };
    create_file_with_content(
        temp_input_dir.path(),
        "globals.json",
        &serde_json::to_string(&globals).unwrap(),
    );

    assert!(copy_globals(
        temp_input_dir.path().to_str().unwrap(),
        temp_output_dir.path().to_str().unwrap()
    )
    .is_ok());

    let output_path = temp_output_dir.path().join("globals.json");
    let output_contents = fs::read_to_string(output_path).unwrap();
    let output_globals: Globals = serde_json::from_str(&output_contents).unwrap();

    assert_eq!(output_globals, globals);
}

#[test]
fn test_file_not_found() {
    let temp_input_dir = TempDir::new("").unwrap();
    let temp_output_dir = TempDir::new("").unwrap();

    assert!(copy_globals(
        temp_input_dir.path().to_str().unwrap(),
        temp_output_dir.path().to_str().unwrap()
    )
    .is_err());
}

#[test]
fn test_invalid_json_format() {
    let temp_input_dir = TempDir::new("").unwrap();
    let temp_output_dir = TempDir::new("").unwrap();
    create_file_with_content(
        temp_input_dir.path(),
        "globals.json",
        "This is not valid JSON",
    );

    assert!(copy_globals(
        temp_input_dir.path().to_str().unwrap(),
        temp_output_dir.path().to_str().unwrap()
    )
    .is_err());
}
