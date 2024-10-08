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
}
