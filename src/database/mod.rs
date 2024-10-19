pub mod data;
pub mod handler;
pub mod repository;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type DatabaseConnection = Arc<Mutex<Connection>>;

pub struct DatabaseHandler {
    connection: DatabaseConnection,
}
