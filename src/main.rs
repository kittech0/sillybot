#![feature(type_alias_impl_trait)]

mod bot;
mod util;
use bot::{database::DatabaseHandler, BotHandler};
use util::Logger;

#[tokio::main]
async fn main() -> util::ErrorResult {
    Logger::init()?;
    let _ = DatabaseHandler::get_connection().await;
    BotHandler.run().await?;
    Ok(())
}
