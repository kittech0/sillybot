mod bot;
mod util;
use bot::{database::DatabaseHandler, BotHandler};

#[tokio::main]
async fn main() -> util::ErrorResult {
    util::logger::Logger::init()?;
    let _ = DatabaseHandler::get_connection().await;
    BotHandler.run().await?;
    Ok(())
}
