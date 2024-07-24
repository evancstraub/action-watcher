use notify::{RecursiveMode, Result, Watcher, INotifyWatcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
impl FileWatcher {
    pub fn new<P: AsRef<Path>>(paths: &[P]) -> Result<Self> {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(move |res| tx.send(res).unwrap())?;
        let mut watched_paths = Vec::new();

        for path in paths {
            let path_buf = path.as_ref().to_path_buf();
            watcher.watch(&path_buf, RecursiveMode::Recursive)?;
            watched_paths.push(path_buf);
        }
        Ok(FileWatcher { watcher, rx , watched_paths })
    }

    pub fn close(&mut self) -> Result<()> {
        let mut last_error = None;
        for path in self.watched_paths.drain(..) {
            if let Err(e) = self.watcher.unwatch(&path) {
                eprintln!("Error unwatching path {:?}: {:?}", path, e);
                last_error = Some(e);
            }
        }
        last_error.map_or(Ok(()), Err)

    }


    pub fn wait_for_event(&self, timeout: Duration) -> Option<notify::Event> {
        self.rx.recv_timeout(timeout).ok().and_then(|res| res.ok())
    }

    pub fn from_config(config: &Config) -> Result<Self> {
        Self::new(&config.watch_paths)
    }
}
use std::time::Duration;

use crate::Config;

pub struct FileWatcher {
    watcher: INotifyWatcher,
    rx: std::sync::mpsc::Receiver<notify::Result<notify::Event>>,
    watched_paths: Vec<PathBuf>,
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("Error closing FileWatcher during drop: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;
    use std::thread;


    fn wait_for_fs() {
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_file_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = FileWatcher::new(&[temp_dir.path()]).expect("Watcher failed to watch file");

        let file_path = temp_dir.path().join("test.txt");
        fs::File::create(&file_path).expect("failed to create file");
        wait_for_fs();

        if let Some(event) = watcher.wait_for_event(Duration::from_secs(1)) {
            assert_eq!(
                event.kind,
                notify::EventKind::Create(notify::event::CreateKind::File)
            );
            assert_eq!(event.paths[0], file_path);
        } else {
            panic!("No event received");
        }

        watcher.close().unwrap()
    }

    #[test]
    fn test_file_modification() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"initial content").unwrap();

        let mut watcher = FileWatcher::new(&[temp_dir.path()]).unwrap();

        fs::write(&file_path, "modified content").unwrap();
        wait_for_fs();

        if let Some(event) = watcher.wait_for_event(Duration::from_secs(1)) {
            assert_eq!(
                event.kind,
                notify::EventKind::Modify(notify::event::ModifyKind::Data(
                    notify::event::DataChange::Any
                ))
            );
            assert_eq!(event.paths[0], file_path);
        } else {
            panic!("No event received");
        }

        watcher.close().unwrap()
    }

    #[test]
    fn test_file_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::File::create(&file_path).unwrap();

        let mut watcher = FileWatcher::new(&[temp_dir.path()]).unwrap();

        fs::remove_file(&file_path).unwrap();
        wait_for_fs();

        if let Some(event) = watcher.wait_for_event(Duration::from_secs(1)) {
            assert_eq!(
                event.kind,
                notify::EventKind::Remove(notify::event::RemoveKind::File)
            );
            assert_eq!(event.paths[0], file_path);

        } else {
            panic!("No event received");
        }
        watcher.close().unwrap()
    }

    #[test]
    fn test_multiple_events() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = FileWatcher::new(&[temp_dir.path()]).expect("failed to watch dir");

        let file_path1 = temp_dir.path().join("test1.txt");
        let file_path2 = temp_dir.path().join("test2.txt");

        fs::File::create(&file_path1).expect("failed to create file");
        wait_for_fs();
        fs::File::create(&file_path2).expect("failed to create file");
        wait_for_fs();

        let mut events = Vec::new();
        for _ in 0..2 {
            if let Some(event) = watcher.wait_for_event(Duration::from_secs(1)) {
                events.push(event);
            }
        }

        assert_eq!(events.len(), 2);
        // assert!(events.iter().any(|e| e.paths[0] == file_path1));
        // assert!(events.iter().any(|e| e.paths[0] == file_path2));
        watcher.close().unwrap()
    }
}
