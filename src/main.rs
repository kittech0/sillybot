#![feature(type_alias_impl_trait)]

mod bot;
mod util;
use bot::{database::DatabaseHandler, BotHandler};
use util::logger;
pub use tokio::fs as tfs;

#[tokio::main]
async fn main() -> util::ErrorResult {
    logger::init()?;
    let _ = DatabaseHandler::get_connection().await;
    BotHandler.run().await?;
    Ok(())
}
