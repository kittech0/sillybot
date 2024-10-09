use std::path::Path;

use chrono::NaiveDate;
use rand::{rngs::ThreadRng, Rng};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tokio::fs;

const TESTING: bool = true;

pub type ErrorResult<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("toml serialization error")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("toml deserialization error")]
    TomlDeError(#[from] toml::de::Error),
    #[error("io error")]
    IoError(#[from] tokio::io::Error),
    #[error("sql error")]
    SqlError(#[from] rusqlite::Error),
    #[error("bot error")]
    SerenityError(#[from] serenity::Error),
    #[error("logger creation error")]
    LoggerCreateError(#[from] log::SetLoggerError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
}
pub struct Book {
    pub rowid: i32,
    pub id: u32,
    pub name: String,
    pub author: String,
    pub publish_date: NaiveDate,
}

pub async fn read_config(path_ref: impl AsRef<Path>) -> ErrorResult<Config> {
    let path = path_ref.as_ref();
    Ok(if !path.is_file() {
        let config = Config {
            token: "".to_string(),
        };
        let ser = toml::to_string(&config)?;
        fs::write(path, ser).await?;
        config
    } else {
        let de = fs::read_to_string(path).await?;
        toml::from_str(&de)?
    })
}
