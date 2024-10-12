pub mod dbhandler;
pub mod userdb;
use std::sync::Arc;

use chrono::NaiveDateTime;
use rusqlite::Connection;
use tokio::sync::Mutex;

pub type DatabaseConnection = Arc<Mutex<Connection>>;
pub struct DatabaseHandler();

#[derive(Debug)]
pub struct User {
    discord_id: u64,
    join_date: NaiveDateTime,
}
