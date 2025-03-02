use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

pub enum RotireMode {
    /// Delete mode deletes files.
    Delete,
}

pub struct Rotire {
    /// Is a flag indicating if rotire executes an operation.
    is_running: AtomicBool,
    /// The directory on which rotire operates on
    directory: PathBuf,
}

pub struct RotireResult {}

impl Rotire {
    pub fn new<P: AsRef<Path>>(directory: P) -> Self {
        Rotire {
            is_running: AtomicBool::new(false),
            directory: directory.as_ref().to_path_buf(),
        }
    }

    pub fn run(&self, keep_max_files: i32) -> Result<RotireResult> {
        self.is_running.store(true, Ordering::Relaxed);
        Ok(RotireResult {})
    }
}
