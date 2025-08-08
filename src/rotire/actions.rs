use std::fs;
use std::fs::File;
use std::os::linux::fs::MetadataExt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use flate2::Compression;
use flate2::write::GzEncoder;
use log::{error, info};
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


impl Action {

    /// Constructs a new action
    pub fn new(action_type: ActionType, dry_run: bool) -> Action {
        Action {
            action_type,
            dry_run,
        }
    }

    pub fn execute(&self, files: Vec<&RotireFile>, directory: PathBuf) -> anyhow::Result<RotireResult> {
        match self.action_type {
            ActionType::Delete => {
                if self.dry_run {
                    dry_run_action(files, String::from("Delete"))
                } else {
                    delete_files_action(files)
                }
            },
            ActionType::Archive => {
                if self.dry_run {
                    dry_run_action(files, String::from("Archive"))
                } else {
                    archive_files_action(files, directory)
                }
            },
        }
    }
}

/// The delete files action is a destructive action which deletes files from the disk.
fn delete_files_action(files: Vec<&RotireFile>) -> anyhow::Result<RotireResult> {
    let mut result = RotireResult::new();
    for file in files {
        result.inc_affected_files_size(file.metadata.st_size());
        result.inc_affected_files();

        if let Err(err) = fs::remove_file(file.path.clone()) {
            error!("failed to delete file {:?}: {}", file.path, err)
        }
    }
    Ok(result)
}

/// The dry run action only gather stats and logs which action has been called with.
fn dry_run_action(files: Vec<&RotireFile>, name: String)-> anyhow::Result<RotireResult> {
    let mut result = RotireResult::new();
    for file in files {
        result.inc_affected_files_size(file.metadata.st_size());
        result.inc_affected_files();

        info!("[DRY RUN] {} on {:?}", name, file.path);
    }
    Ok(result)
}

fn archive_files_action(files: Vec<&RotireFile>, directory: PathBuf) -> anyhow::Result<RotireResult> {
    let mut result = RotireResult::new();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let mut archive_path = PathBuf::from(directory);
    archive_path = archive_path.join(format!("rotire-archive-{0}.tar.gz", timestamp));

    let tar_gz = File::create(archive_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for file in files {
        result.inc_affected_files_size(file.metadata.st_size());
        result.inc_affected_files();

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
    tar.finish()?;
    Ok(result)
}


impl Default for Action {
    fn default() -> Action {
        Action {
            action_type: ActionType::Archive,
            dry_run: false,
        }
    }
}