use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::{File, Metadata};
use std::os::linux::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use flate2::Compression;
use flate2::write::GzEncoder;
use log::error;

/// RotireAction is the action that rotire will perform on run.
pub enum RotireAction {
    /// Delete mode deletes files.
    Delete,
    /// Archive files and delete older ones.
    ArchiveAndDelete
}

#[derive(Debug)]
pub struct Rotire {
    /// Is a flag indicating if rotire executes an operation.
    is_running: AtomicBool,
    /// The directory on which rotire operates on
    directory: PathBuf,
}

#[derive(Debug)]
/// RotireFile is a special file used within rotire that contains a path and a metadata.
pub struct RotireFile {
    pub path: PathBuf,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub struct RotireResult {
    /// affected_files represents the number of affected files
    pub affected_files: i32,
    pub affected_files_size: u64,
}

impl RotireResult {
    fn new() -> Self {
        RotireResult {
            affected_files: 0,
            affected_files_size: 0,
        }
    }

    /// Increments the affected files counter.
    fn inc_affected_files(&mut self) {
        self.affected_files += 1
    }

    /// Increments the affected files size.
    fn inc_affected_files_size(&mut self, size: u64) {
        self.affected_files_size += size
    }
}

impl Display for RotireResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let size_in_mib = self.affected_files_size / (1024 * 1024);
        write!(
            f,
            "Affected files {}, containing {} MiB.",
            self.affected_files, size_in_mib
        )
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

    fn execute_action(&self, files: Vec<&RotireFile>, action: RotireAction) -> Result<RotireResult> {
        let mut result = RotireResult::new();
        match action {
            RotireAction::Delete => {
                for file in files {
                    result.inc_affected_files_size(file.metadata.st_size());
                    result.inc_affected_files();

                    // do action
                    if let Err(result) = fs::remove_file(file.path.clone()) {
                        error!("failed to delete file {0:?}: {1}", file.path, result)
                    }
                }
            }
            RotireAction::ArchiveAndDelete => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();

                let mut archive_path = PathBuf::from(self.directory.clone());
                archive_path = archive_path.join(format!("rotire-archive-{0}.tar.gz", timestamp));
                let tar_gz = File::create(archive_path)?;
                let enc = GzEncoder::new(tar_gz, Compression::default());
                let mut tar = tar::Builder::new(enc);
                files.iter().for_each(|file| {
                    result.inc_affected_files_size(file.metadata.st_size());
                    result.inc_affected_files();

                    // do action
                    let file_path: &PathBuf = &file.path.clone();
                    let file_result = File::open(file_path);
                    match file_result {
                        Ok(mut file_handle) => {
                            if let Err(result) = tar.append_file(file_path.file_name().unwrap(), &mut file_handle) {
                                error!("failed to archive file {0:?}: {1}", file_path, result)
                            } else {
                                if let Err(result) = fs::remove_file(file_path) {
                                    error!("failed to delete file {0:?}: {1}", file_path, result)
                                }
                            }
                        }
                        Err(err) => {
                            error!("failed to open file {0:?}: {1}", file_path, err)
                        }
                    }


                });
                tar.finish()?;

            }
        }
        Ok(result)
    }

    pub fn run(&self, keep_max_files: i32, action: RotireAction) -> Result<RotireResult> {
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
        let result = self.execute_action(files.iter().rev().skip(keep_max_files as usize).collect(), action)?;

        self.is_running.store(false, Ordering::Relaxed);
        Ok(result)
    }
}
