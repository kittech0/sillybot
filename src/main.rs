#![feature(type_alias_impl_trait)]
mod bot;
mod database;
mod util;
use bot::BotHandler;
use database::{repository::UserRepository, DatabaseHandler};
use util::logger;

#[tokio::main]
async fn main() -> util::ErrorResult {
    let db = DatabaseHandler::new(Option::None)?;
    UserRepository::init(&db.get_connection()).await?;
    logger::init()?;
    BotHandler::new(db.get_connection()).run().await?;
    Ok(())
}
