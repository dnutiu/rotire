use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

pub enum RotireMode {
    /// Delete mode deletes files.
    Delete,
}

#[derive(Debug)]
pub struct Rotire {
    /// Is a flag indicating if rotire executes an operation.
    is_running: AtomicBool,
    /// The directory on which rotire operates on
    directory: PathBuf,
}

#[derive(Debug)]
pub struct RotireFile {
    pub path: PathBuf,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub struct RotireResult {}

impl Display for RotireResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "All good.")
    }
}

impl Rotire {
    pub fn new<P: AsRef<Path>>(directory: P) -> Self {
        Rotire {
            is_running: AtomicBool::new(false),
            directory: directory.as_ref().to_path_buf(),
        }
    }

    /// Lists the files in the given directory and returns a vector of file metadata.
    fn list_files_in_directory(&self, directory: &PathBuf) -> Result<Vec<RotireFile>> {
        let files: Vec<RotireFile> = fs::read_dir(directory)?
            .into_iter()
            .filter_map(|p| p.ok().map(|ok_path| ok_path.path()))
            .filter_map(|path| {
                let metadata = fs::metadata(path.clone()).ok().map(|m| m);
                if let Some(metadata) = metadata {
                    Some(RotireFile { path, metadata })
                } else {
                    None
                }
            })
            .collect();

        Ok(files)
    }

    pub fn run(&self, keep_max_files: i32) -> Result<RotireResult> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err(anyhow!(
                "Can't start another run while one is already running."
            ));
        }
        self.is_running.store(true, Ordering::Relaxed);

        let files: Vec<RotireFile> = self.list_files_in_directory(&self.directory)?;

        for file in files {
            println!("File {:?}", file)
        }

        // TODO: sort my mtime
        // TODO: delete the rest of files

        self.is_running.store(false, Ordering::Relaxed);
        Ok(RotireResult {})
    }
}
