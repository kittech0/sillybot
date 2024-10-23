pub mod connection;
pub mod data;
pub mod repository;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct DatabaseConnection(pub Arc<Mutex<Connection>>);
