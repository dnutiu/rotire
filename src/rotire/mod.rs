pub mod filter;
mod model;

use anyhow::{anyhow, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{error, info};
use std::fs;
use std::fs::{File};
use std::os::linux::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::rotire::filter::RotireFilter;
use crate::rotire::model::{RotireFile, RotireResult};

/// ActionType is the type of action that rotire will perform on run.
pub enum ActionType {
    /// Delete mode deletes files.
    Delete,
    /// Archive files and delete older ones.
    Archive,
}

/// Action is the action that rotire will perform on run.
pub struct Action {
    /// The type of action to perform.
    pub action_type: ActionType,
    /// Whether to perform a dry run.
    pub dry_run: bool,
}

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

    fn delete_files(&self, files: Vec<&RotireFile>, action: &Action) -> Result<RotireResult> {
        let mut result = RotireResult::new();
        for file in files {
            result.inc_affected_files_size(file.metadata.st_size());
            result.inc_affected_files();

            if action.dry_run {
                info!("[DRY-RUN] Deleting file {:?}", file.path);
            } else {
                if let Err(err) = fs::remove_file(file.path.clone()) {
                    error!("failed to delete file {:?}: {}", file.path, err)
                }
            }
        }
        Ok(result)
    }

    fn archive_files(&self, files: Vec<&RotireFile>, action: &Action) -> Result<RotireResult> {
        let mut result = RotireResult::new();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut archive_path = PathBuf::from(self.directory.clone());
        archive_path = archive_path.join(format!("rotire-archive-{0}.tar.gz", timestamp));

        if action.dry_run {
            info!("[DRY-RUN] Archiving to {:?}", archive_path);
        }

        let tar_gz = File::create(archive_path)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);

        for file in files {
            result.inc_affected_files_size(file.metadata.st_size());
            result.inc_affected_files();

            if action.dry_run {
                info!("[DRY-RUN] Adding file {:?} to archive", file.path);
            } else {
                let file_path: &PathBuf = &file.path.clone();
                let file_result = File::open(file_path);
                match file_result {
                    Ok(mut file_handle) => {
                        if let Err(err) =
                            tar.append_file(file_path.file_name().unwrap(), &mut file_handle)
                        {
                            error!("failed to archive file {:?}: {}", file_path, err)
                        } else {
                            if let Err(err) = fs::remove_file(file_path) {
                                error!("failed to delete file {:?}: {}", file_path, err)
                            }
                        }
                    }
                    Err(err) => {
                        error!("failed to open file {:?}: {}", file_path, err)
                    }
                }
            }
        }
        tar.finish()?;
        Ok(result)
    }

    fn execute_action(
        &self,
        files: Vec<&RotireFile>,
        action: &Action,
    ) -> Result<RotireResult> {
        match action.action_type {
            ActionType::Delete => self.delete_files(files, action),
            ActionType::Archive => self.archive_files(files, action),
        }
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