use std::{fmt::Display, path::Path, process::exit};

use tokio::fs::{self, File};

use super::ErrorResult;

pub async fn read_token(path_ref: impl AsRef<Path>) -> ErrorResult<Option<String>> {
    let path = path_ref.as_ref();
    Ok(if !path.is_file() {
        File::create(path).await?;
        None
    } else {
        Some(fs::read_to_string(path).await?)
    })
}

pub fn throw_error(message: impl Display) -> ! {
    log::error!("{message}");
    exit(1)
}
