use penumbra_registry::parser::get_chain_configs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempdir::TempDir;

fn create_test_config_file(dir: &Path, file_name: &str, contents: &str) {
    let file_path = dir.join(file_name);
    let mut file = File::create(file_path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

#[test]
fn test_get_chain_configs_reads_configs_correctly() {
    let temp_input_dir = TempDir::new("test_input").unwrap();
    let temp_registry_dir = TempDir::new("test_registry").unwrap();

    let config_content = serde_json::json!({
        "chainId": "test-chain-1",
        "rpcs": [],
        "ibcConnections": [],
        "nativeAssets": []
    })
    .to_string();
    create_test_config_file(temp_input_dir.path(), "test-chain-1.json", &config_content);

    let configs = get_chain_configs(
        temp_registry_dir.path().to_str().unwrap(),
        temp_input_dir.path().to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].chain_id, "test-chain-1");
}

#[test]
fn test_get_chain_configs_reads_multiple_configs_correctly() {
    let temp_input_dir = TempDir::new("test_input").unwrap();
    let temp_registry_dir = TempDir::new("test_registry").unwrap();

    let config_content_1 = serde_json::json!({
        "chainId": "test-chain-1",
        "rpcs": [],
        "ibcConnections": [],
        "nativeAssets": []
    })
    .to_string();
    create_test_config_file(
        temp_input_dir.path(),
        "test-chain-1.json",
        &config_content_1,
    );

    let config_content_2 = serde_json::json!({
        "chainId": "test-chain-2",
        "rpcs": [],
        "ibcConnections": [],
        "nativeAssets": []
    })
    .to_string();
    create_test_config_file(
        temp_input_dir.path(),
        "test-chain-2.json",
        &config_content_2,
    );

    let configs = get_chain_configs(
        temp_registry_dir.path().to_str().unwrap(),
        temp_input_dir.path().to_str().unwrap(),
    )
    .unwrap();

    assert_eq!(configs.len(), 2);
    assert!(configs
        .iter()
        .any(|config| config.chain_id == "test-chain-1"));
    assert!(configs
        .iter()
        .any(|config| config.chain_id == "test-chain-2"));
}

#[test]
fn test_get_chain_configs_handles_invalid_json() {
    let temp_input_dir = TempDir::new("test_input").unwrap();
    let temp_registry_dir = TempDir::new("test_registry").unwrap();

    let invalid_config_content = "{ invalid json }";
    create_test_config_file(
        temp_input_dir.path(),
        "invalid.json",
        invalid_config_content,
    );

    let configs = get_chain_configs(
        temp_registry_dir.path().to_str().unwrap(),
        temp_input_dir.path().to_str().unwrap(),
    )
    .unwrap();

    assert_eq!(configs.len(), 0);
}
