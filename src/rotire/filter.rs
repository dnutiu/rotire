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