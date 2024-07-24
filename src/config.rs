use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub watch_paths: Vec<String>,
    pub commands: Vec<String>,
    pub report_dir: String,
}

impl Config {
    pub fn from_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_yaml() {
        let yaml = r#"
        watch_paths:
          - "./src"
          - "./tests"
        commands:
          - "cargo test"
          - "cargo clippy"
        report_dir: "./reports"
        "#;

        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.watch_paths, vec!["./src", "./tests"]);
        assert_eq!(config.commands, vec!["cargo test", "cargo clippy"]);
        assert_eq!(config.report_dir, "./reports");
    }
}
