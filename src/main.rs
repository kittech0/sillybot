mod bot;
mod commands;
mod logger;
use bot::BotHandler;
use logger::Logger;
use sillybot::ErrorResult;

#[tokio::main]
async fn main() -> ErrorResult {
    let logger = Logger::new();
    logger.init()?;
    BotHandler::run().await?;
    //sql::sql()?;
    Ok(())
}
