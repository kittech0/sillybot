pub mod data;
pub mod dbhandler;
use std::sync::Arc;
pub mod repositories;
use rusqlite::Connection;
use tokio::sync::Mutex;

pub type DatabaseConnection = Arc<Mutex<Connection>>;
pub struct DatabaseHandler;

pub struct UserRepository(DatabaseConnection);
