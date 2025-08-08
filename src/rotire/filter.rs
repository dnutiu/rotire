use crate::rotire::model::RotireFile;

/// Filters filter file names based on some rules.
#[derive(Debug)]
pub enum RotireFilter {
    Prefix { value: String },
    Suffix { value: String },
}

impl RotireFilter {
    pub fn satisfies(&self, file: &RotireFile) -> bool {
        match self {
            RotireFilter::Prefix { value } => file
                .path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(value),
            RotireFilter::Suffix { value } => file
                .path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .ends_with(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rotire::model::RotireFile;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_temp_file() -> File {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        let file_path = dir_path.join(format!("file{}", 1));
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        file
    }

    #[test]
    fn test_rotire_filter_suffix_ok() -> Result<(), String> {
        let filter = RotireFilter::Suffix {
            value: ".foo".to_string(),
        };
        let temp_file = create_temp_file();

        let file = RotireFile {
            path: PathBuf::from("test.foo"),
            metadata: temp_file.metadata().unwrap(),
        };

        assert_eq!(filter.satisfies(&file), true);
        Ok(())
    }

    #[test]
    fn test_rotire_filter_suffix_nok() -> Result<(), String> {
        let filter = RotireFilter::Suffix {
            value: ".foo".to_string(),
        };
        let temp_file = create_temp_file();

        let file = RotireFile {
            path: PathBuf::from("test.bar"),
            metadata: temp_file.metadata().unwrap(),
        };

        assert_eq!(filter.satisfies(&file), false);
        Ok(())
    }

    #[test]
    fn test_rotire_filter_prefix_ok() -> Result<(), String> {
        let filter = RotireFilter::Prefix {
            value: "some-file-".to_string(),
        };
        let temp_file = create_temp_file();

        let file = RotireFile {
            path: PathBuf::from("some-file-test.foo"),
            metadata: temp_file.metadata().unwrap(),
        };

        assert_eq!(filter.satisfies(&file), true);
        Ok(())
    }

    #[test]
    fn test_rotire_filter_prefix_nok() -> Result<(), String> {
        let filter = RotireFilter::Prefix {
            value: "some-file-".to_string(),
        };
        let temp_file = create_temp_file();

        let file = RotireFile {
            path: PathBuf::from("some-fileis-test.foo"),
            metadata: temp_file.metadata().unwrap(),
        };

        assert_eq!(filter.satisfies(&file), false);
        Ok(())
    }
}
