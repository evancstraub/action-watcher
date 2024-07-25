// src/command_runner.rs

use crate::Config;
use std::process::Command;
pub struct CommandRunner {
    pub commands: Vec<String>,
}

impl CommandRunner {
    pub fn new(cmds: &Vec<String>) -> Self {
        let mut commands: Vec<String> = Vec::new();

        for cmd in cmds {
            let command = cmd.as_str();
            commands.push(command.to_string())
        }

        CommandRunner { commands }
    }
    pub fn run_commands(&self) -> Vec<CommandResult> {
        self.commands
            .iter()
            .map(|cmd| Self::run_command(cmd))
            .collect()
    }

    pub fn from_config(config: &Config) -> Self {
        Self::new(&config.commands)
    }

    fn run_command(command: &str) -> CommandResult {
        println!("Running command: {}", command);

        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output();

        match output {
            Ok(output) => {
                let status = output.status.code().unwrap_or(-1);
                let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

                    CommandResult {
                        command: command.to_string(),
                        status,
                        stdout,
                        stderr,
                    }


            },
            Err(e) => CommandResult {
                command: command.to_string(),
                status: -1,
                stdout: String::new(),
                stderr: format!("Failed to execute command: {}", e),
            }
        }
    }
}

pub struct CommandResult {
    pub command: String,
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use tempfile::TempDir;

    fn create_temp_config(commands: Vec<String>, report_dir: &Path) -> Config {
        Config {
            watch_paths: vec![],
            commands,
            report_dir: "".parse().unwrap(),
        }
    }

    #[test]
    fn test_run_command_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_output.txt");
        let command = format!("echo 'Hello, World!' > {}", file_path.to_str().unwrap());

        let result = CommandRunner::run_command(&command);

        assert_eq!(result.status, 0);
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());

        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Hello, World!\n");
    }

    #[test]
    fn test_run_command_failure() {
        let temp_dir = TempDir::new().unwrap();
        let command = "non_existent_command";

        let result = CommandRunner::run_command(&command);

        assert_ne!(result.status, 0);
        assert!(result.stdout.is_empty());
        // assert!(result.stderr.contains("command not found") || result.stderr.contains("not recognized"));
    }

}
