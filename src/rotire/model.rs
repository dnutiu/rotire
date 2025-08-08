use std::fmt::{Display, Formatter};
use std::fs::Metadata;
use std::path::PathBuf;

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
    pub fn new() -> Self {
        RotireResult {
            affected_files: 0,
            affected_files_size: 0,
        }
    }

    /// Increments the affected files counter.
    pub fn inc_affected_files(&mut self) {
        self.affected_files += 1
    }

    /// Increments the affected files size.
    pub fn inc_affected_files_size(&mut self, size: u64) {
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
