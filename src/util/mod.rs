pub mod funcs;
pub mod logger;

const _TESTING: bool = true;

pub type ErrorResult<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] tokio::io::Error),
    #[error("sql error")]
    Sql(#[from] rusqlite::Error),
    #[error("bot error")]
    Serenity(#[from] serenity::Error),
    #[error("logger creation error")]
    LoggerCreate(#[from] log::SetLoggerError),
}
