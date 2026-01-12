use std::fmt::{Display, Formatter};
use derive_more::From;
use crate::fs;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, From)]
pub enum Error {

    // -- fs
    #[from]
    Fs(fs::Error),

    #[from]
    Io(std::io::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}