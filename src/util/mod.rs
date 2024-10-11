pub mod logger;
use std::path::Path;

use tokio::fs::{self, File};

const _TESTING: bool = true;

pub type ErrorResult<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] tokio::io::Error),
    #[error("sql error")]
    Sql(#[from] tokio_rusqlite::Error),
    #[error("bot error")]
    Serenity(#[from] serenity::Error),
    #[error("logger creation error")]
    LoggerCreate(#[from] log::SetLoggerError),
}

pub async fn read_token(path_ref: impl AsRef<Path>) -> ErrorResult<Option<String>> {
    let path = path_ref.as_ref();
    Ok(if !path.is_file() {
        File::create(path).await?;
        None
    } else {
        Some(fs::read_to_string(path).await?)
    })
}

