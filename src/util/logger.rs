use std::fs::{self, File};

use chrono::{Datelike, Local, Timelike};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

use super::ErrorResult;

pub fn init() -> ErrorResult {
    let date_now = Local::now();
    let file_path = format!(
        "./logs/{}-{}-{}.{}-{}-{}.log",
        date_now.year(),
        date_now.month(),
        date_now.day(),
        date_now.hour(),
        date_now.minute(),
        date_now.second()
    );
    fs::create_dir_all("./logs")?;
    Ok(CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create(&file_path)?,
        ),
    ])?)
}
