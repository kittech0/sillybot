#![feature(type_alias_impl_trait)]
mod bot;
mod database;
mod util;
use bot::BotHandler;
use database::{
    repository::{MessageRepository, UserRepository},
    DatabaseConnection,
};
use util::logger;

#[tokio::main]
async fn main() -> util::ErrorResult {
    logger::init()?;

    let db = DatabaseConnection::new(Option::None)?;
    UserRepository::init(db.clone()).await?;
    MessageRepository::init(db.clone()).await?;
    BotHandler::new(db).run().await?;
    Ok(())
}
