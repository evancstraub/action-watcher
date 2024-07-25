use action_watcher::{CommandRunner, Config, FileWatcher};
use std::time::Duration;

fn main() {
    let config_result = Config::from_yaml("examples/example.yaml");

    let config = match config_result {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {:?}", e);
            return;
        }
    };

    let command_runner = CommandRunner::from_config(&config);

    let file_watcher_result = FileWatcher::from_config(&config);

    let mut file_watcher = match file_watcher_result {
        Ok(watcher) => watcher,
        Err(e) => {
            eprintln!("Failed to create file watcher: {:?}", e);
            return;
        }
    };

    loop {
        if let Some(event) = file_watcher.wait_for_event(Duration::from_secs(1)) {
            println!("File event detected: {:?}", event);
            let results = command_runner.run_commands();
            for result in results {
                println!("{}", result.stderr);
            }
            // file_watcher.run_commands();
        }
    }
}
