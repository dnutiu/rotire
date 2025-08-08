use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_rotire_archive_action() {
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create some test files with varying modification times
    for i in 0..10 {
        let file_path = dir_path.join(format!("file{}", i));
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        // Sleep to ensure different modification times
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    let rotire_exe = env!("CARGO_BIN_EXE_rotire");

    let output = Command::new(rotire_exe)
        .arg("--directory")
        .arg(dir_path)
        .arg("--keep-n")
        .arg("4")
        .arg("archive")
        .output()
        .expect("Failed to execute rotire");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    let remaining_files: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    // 4 files should be kept, and 1 archive should be created
    assert_eq!(remaining_files.len(), 5);

    let archive_file = remaining_files
        .iter()
        .find(|p| p.extension().unwrap_or_default() == "gz");

    assert!(archive_file.is_some());

    let found_files: Vec<_> = remaining_files
        .iter()
        .map(|p| {
            p.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|p| !p.ends_with(".tar.gz"))
        .collect();

    assert_eq!(found_files, vec!["file9", "file8", "file7", "file6"]);
}

#[test]
fn test_rotire_delete_action() {
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create some test files with varying modification times
    for i in 0..10 {
        let file_path = dir_path.join(format!("file{}", i));
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        // Sleep to ensure different modification times
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    let rotire_exe = env!("CARGO_BIN_EXE_rotire");

    let output = Command::new(rotire_exe)
        .arg("--directory")
        .arg(dir_path)
        .arg("--keep-n")
        .arg("4")
        .arg("delete")
        .output()
        .expect("Failed to execute rotire");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    let remaining_files: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    // 4 files should be kept, and 1 archive should be created
    assert_eq!(remaining_files.len(), 4);

    let found_files: Vec<_> = remaining_files
        .iter()
        .map(|p| {
            p.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    assert_eq!(found_files, vec!["file9", "file8", "file7", "file6"]);
}

#[test]
fn test_rotire_delete_action_dry_run() {
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create some test files with varying modification times
    for i in 0..10 {
        let file_path = dir_path.join(format!("file{}", i));
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        // Sleep to ensure different modification times
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    let rotire_exe = env!("CARGO_BIN_EXE_rotire");

    let output = Command::new(rotire_exe)
        .arg("--directory")
        .arg(dir_path)
        .arg("--keep-n")
        .arg("4")
        .arg("delete")
        .arg("--dry-run")
        .output()
        .expect("Failed to execute rotire");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    let remaining_files: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    assert_eq!(remaining_files.len(), 10);

    let found_files: Vec<_> = remaining_files
        .iter()
        .map(|p| {
            p.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    assert_eq!(
        found_files,
        vec![
            "file9", "file8", "file7", "file6", "file5", "file4", "file3", "file2", "file1",
            "file0"
        ]
    );
}
