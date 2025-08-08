pub mod filter;
pub mod model;
pub mod actions;

use crate::rotire::filter::RotireFilter;
use crate::rotire::model::{RotireFile, RotireResult};
use anyhow::{anyhow, Result};

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::rotire::actions::{Action};

#[derive(Debug)]
pub struct Rotire {
    /// Is a flag indicating if rotire executes an operation.
    is_running: AtomicBool,
    /// The directory on which rotire operates on
    directory: PathBuf,
    filters: Vec<RotireFilter>,
}

impl Rotire {
    pub fn new<P: AsRef<Path>>(directory: P) -> Self {
        Rotire {
            is_running: AtomicBool::new(false),
            directory: directory.as_ref().to_path_buf(),
            filters: Vec::default(),
        }
    }

    /// Adds a rotire filter to this instance.
    pub fn add_filter(&mut self, filter: RotireFilter) {
        self.filters.push(filter);
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
            .filter(|file| {
                if self.filters.is_empty() {
                    return true;
                }
                return self.filters.iter().all(|x| x.satisfies(file));
            })
            .collect();

        Ok(files)
    }
    
    
    fn execute_action(&self, files: Vec<&RotireFile>, action: &Action) -> Result<RotireResult> {
        action.execute(files, self.directory.clone())
    }

    pub fn run(&self, keep_max_files: i32, action: Action) -> Result<RotireResult> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err(anyhow!(
                "Can't start another run while one is already running."
            ));
        }
        self.is_running.store(true, Ordering::Relaxed);

        let mut files: Vec<RotireFile> = self.list_files_in_directory(&self.directory)?;

        // Sort files by modified time
        files.sort_unstable_by(
            |a, b| match (a.metadata.modified(), b.metadata.modified()) {
                (Ok(a_time), Ok(b_time)) => a_time.cmp(&b_time),
                (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                (Err(_), Ok(_)) => std::cmp::Ordering::Less,
                (Err(_), Err(_)) => std::cmp::Ordering::Less,
            },
        );

        // Execute action and record result
        let result = self.execute_action(
            files.iter().rev().skip(keep_max_files as usize).collect(),
            &action,
        )?;

        self.is_running.store(false, Ordering::Relaxed);
        Ok(result)
    }
}
