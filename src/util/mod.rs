pub mod logger;
use std::path::Path;

use tokio::fs::{self, File};

const _TESTING: bool = true;

pub type ErrorResult<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("toml serialization error")]
    TomlSer(#[from] toml::ser::Error),
    #[error("toml deserialization error")]
    TomlDe(#[from] toml::de::Error),
    #[error("io error")]
    Io(#[from] tokio::io::Error),
    #[error("sql error")]
    Sql(#[from] tokio_rusqlite::Error),
    #[error("bot error")]
    Serenity(#[from] serenity::Error),
    #[error("logger creation error")]
    LoggerCreate(#[from] log::SetLoggerError),
}

pub async fn read_token(path_ref: impl AsRef<Path>) -> ErrorResult<String> {
    let path = path_ref.as_ref();
    Ok(if !path.is_file() {
        File::create(path).await?;
        "".to_string()
    } else {
        fs::read_to_string(path).await?
    })
}