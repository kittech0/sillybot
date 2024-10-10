use std::fs::{self, File};

use chrono::{Datelike, Local, Timelike};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

use super::ErrorResult;

pub struct Logger {
    file_path: String,
}

impl Logger {
    pub fn new() -> Self {
        let date_now = Local::now();
        Self {
            file_path: format!(
                "./logs/{}-{}-{}.{}-{}-{}.log",
                date_now.year(),
                date_now.month(),
                date_now.day(),
                date_now.hour(),
                date_now.minute(),
                date_now.second()
            ),
        }
    }

    pub fn init(&self) -> ErrorResult {
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
                File::create(&self.file_path)?,
            ),
        ])?)
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
