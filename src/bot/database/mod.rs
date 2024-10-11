pub mod dbhandler;
pub mod userdb;
use std::sync::Arc;

use chrono::NaiveDateTime;
use tokio_rusqlite::Connection;

pub struct DatabaseHandler {
    connection: Arc<Connection>,
}

#[derive(Debug)]
pub struct User {
    discord_id: u64,
    join_date: NaiveDateTime,
}
