use std::fmt::{Display, Formatter};
use derive_more::From;
use crate::error;

pub fn list_files(path: &str) -> crate::Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
        ;
    if files.is_empty() {
        return Err(error::Error::Fs(Error::SillyOneCantListEmptyFolder));
    }
    Ok(files)
}

#[derive(Debug, From)]
pub enum Error {
    SillyOneCantListEmptyFolder
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
