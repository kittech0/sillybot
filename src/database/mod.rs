pub mod data;
pub mod connection;
pub mod repository;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DatabaseConnection(pub Arc<Mutex<Connection>>);
