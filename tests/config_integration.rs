use action_watcher::Config;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_config_from_file() {
    let yaml = r#"
    watch_paths:
      - "./src"
      - "./tests"
    commands:
      - "cargo test"
      - "cargo clippy"
    report_dir: "./reports"
    "#;

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), yaml).unwrap();

    let config = Config::from_yaml(temp_file.path().to_str().unwrap()).unwrap();
    assert_eq!(config.watch_paths, vec!["./src", "./tests"]);
    assert_eq!(config.commands, vec!["cargo test", "cargo clippy"]);
    assert_eq!(config.report_dir, "./reports");
}
