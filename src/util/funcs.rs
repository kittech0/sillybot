use std::{
    fmt::{Debug, Display},
    fs,
    path::Path,
    process::exit,
};

use super::ErrorResult;

pub fn read_token(path_ref: impl AsRef<Path>) -> ErrorResult<Option<String>> {
    let path = path_ref.as_ref();
    Ok(if !path.is_file() {
        fs::File::create(path)?;
        None
    } else {
        Some(fs::read_to_string(path)?)
    })
}

pub fn throw(message: impl Display, error: impl Debug) -> ! {
    log::error!("{message}: {error:?}");
    exit(1)
}
