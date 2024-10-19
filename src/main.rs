#![feature(type_alias_impl_trait)]
mod bot;
mod database;
mod util;
use bot::BotHandler;
use database::{repository::UserRepository, DatabaseConnection};
use util::logger;

#[tokio::main]
async fn main() -> util::ErrorResult {
    let db = DatabaseConnection::new(Option::None)?;
    UserRepository::init(db.clone()).await?;
    logger::init()?;
    BotHandler::new(db).run().await?;
    Ok(())
}
