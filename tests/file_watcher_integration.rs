
use action_watcher::{Config, FileWatcher}; // Assuming these are exposed in your lib.rs
use std::fs;
use std::path::Path;
use std::time::Duration;
use tempfile::TempDir;



fn create_test_config(dirs: &[&Path]) -> Config {
    Config {
        watch_paths: dirs.iter().map(|dir| dir.to_str().unwrap().to_string()).collect(),
        commands: vec!["echo 'File changed'".to_string()],
        report_dir: dirs[0].join("reports").to_str().unwrap().to_string(),
    }
}

#[test]
fn test_file_watcher_integration() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&[temp_dir.path()]);

    let file_watcher = FileWatcher::from_config(&config).expect("Failed to create FileWatcher");

    // Create a new file
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "Initial content").expect("Failed to write file");

    // Wait for the file creation event
    let event = file_watcher.wait_for_event(Duration::from_secs(2));
    assert!(event.is_some(), "No event received for file creation");
    let event = event.unwrap();
    assert_eq!(event.paths[0], test_file);
    // assert!(matches!(event.kind, notify::EventKind::Create(_)));

    // Modify the file
    fs::write(&test_file, "Modified content").expect("Failed to modify file");

    // Wait for the file modification event
    let event = file_watcher.wait_for_event(Duration::from_secs(2));
    assert!(event.is_some(), "No event received for file modification");
    let event = event.unwrap();
    assert_eq!(event.paths[0], test_file);
    // assert!(matches!(event.kind, notify::EventKind::Modify(_)));

    // Delete the file
    fs::remove_file(&test_file).expect("Failed to delete file");

    // Wait for the file deletion event
    let event = file_watcher.wait_for_event(Duration::from_secs(2));
    assert!(event.is_some(), "No event received for file deletion");
    let event = event.unwrap();
    assert_eq!(event.paths[0], test_file);
    // assert!(matches!(event.kind, notify::EventKind::Remove(_)));
}

// #[test]
// fn test_file_watcher_with_commands() {
//     let temp_dir = TempDir::new().unwrap();
//     let config = create_test_config(&[temp_dir.path()]);
//
//     // Use a command that creates a file when executed
//     // config.commands = vec![format!("touch {}", temp_dir.path().join("command_executed.txt").to_str().unwrap())];
//
//     let file_watcher = FileWatcher::from_config(&config).expect("Failed to create FileWatcher");
//
//     // Create a new file to trigger the watcher
//     let test_file = temp_dir.path().join("trigger.txt");
//     fs::write(&test_file, "Trigger content").expect("Failed to write trigger file");
//
//     // Wait for the file creation event and command execution
//     file_watcher.wait_for_event(Duration::from_secs(2));
//
//     // Check if the command was executed
//     let command_file = temp_dir.path().join("command_executed.txt");
//     assert!(command_file.exists(), "Command was not executed");
// }