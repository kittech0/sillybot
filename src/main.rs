mod bot;
mod commands;
mod logger;
mod sql;
use bot::BotHandler;
use chrono::{DateTime, Datelike, Local, Timelike};
use log::LevelFilter;
use logger::Logger;
use owo_colors::OwoColorize;
use sillybot::ErrorResult;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::{fs::File, io::Write};

#[tokio::main]
async fn main() -> ErrorResult {
    let logger = Logger::new();
    logger.run()?;
    log::warn!("test");
    //BotHandler::run().await?;
    //sql::sql()?;
    Ok(())
}
