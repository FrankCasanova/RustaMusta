use std::path::Path;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::time::Duration;
use notify::{Watcher, RecursiveMode, RecommendedWatcher, recommended_watcher};

pub struct FileWatcher {
    receiver: Receiver<notify::Result<notify::Event>>,
    _watcher: RecommendedWatcher, // We keep this to avoid it being dropped
}

impl FileWatcher {
    pub fn new(path: &str) -> Result<Self, notify::Error> {
        let (sender, receiver) = channel();
        let mut watcher = recommended_watcher(sender)?;
        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
        
        Ok(FileWatcher {
            receiver,
            _watcher: watcher,
        })
    }
    
    pub fn check_for_changes(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => true,
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => false,
        }
    }
}