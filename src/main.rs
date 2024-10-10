mod bot;
mod util;
use bot::BotHandler;

#[tokio::main]
async fn main() -> util::ErrorResult {
    util::logger::Logger::new().init()?;
    BotHandler::new().run().await?;
    Ok(())
}
