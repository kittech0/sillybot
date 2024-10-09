use std::{
    fs::{self, File},
    path::Path,
};

use chrono::{DateTime, Datelike, Local, Timelike};
use log::LevelFilter;
use sillybot::ErrorResult;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

pub struct Logger {
    date_now: DateTime<Local>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            date_now: Local::now(),
        }
    }

    pub fn init(&self) -> ErrorResult {
        fs::create_dir_all("./logs")?;
        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Warn,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Trace,
                Config::default(),
                File::create(format!(
                    "./logs/{}-{}-{}.{}-{}-{}.log",
                    self.date_now.year(),
                    self.date_now.month(),
                    self.date_now.day(),
                    self.date_now.hour(),
                    self.date_now.minute(),
                    self.date_now.second()
                ))?,
            ),
        ])?;
        Ok(())
    }
}

// impl Drop for Logger {
//     fn drop(&mut self) {
//         let file_path = format!(
//             "{}-{}-{}.{}-{}-{}",
//             self.date_now.year(),
//             self.date_now.month(),
//             self.date_now.day(),
//             self.date_now.hour(),
//             self.date_now.minute(),
//             self.date_now.second()
//         );
//         let mut log_file = File::open(format!("{file_path}.log")).unwrap();
//         let zstd_file =
//             File::create(format!("{file_path}.tar.zst")).expect("Unable to open & create file");
//         let mut enc = Encoder::new(zstd_file, 9).unwrap();
//         let mut tar = Builder::new(&mut enc);

//         tar.append_file(format!("{file_path}.log"), &mut log_file)
//             .unwrap();
//         tar.finish().unwrap();
//     }
// }
