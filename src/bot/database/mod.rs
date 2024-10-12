pub mod dbhandler;
pub mod userdb;
use std::sync::Arc;

use chrono::NaiveDateTime;
use rusqlite::Connection;
use tokio::sync::Mutex;

pub struct DatabaseHandler(Arc<Mutex<Connection>>);

#[derive(Debug)]
pub struct User {
    discord_id: u64,
    join_date: NaiveDateTime,
}
