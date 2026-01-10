
mod fs;
mod error;

pub use error::{Error, Result};

use crate::fs::list_files;

fn main() -> Result<()>{
    let files = list_files(".")?;
    println!("{files:#?}");
    Ok(())
}

