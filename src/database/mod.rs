pub mod connection;
pub mod data;
pub mod repository;
use rusqlite::Connection;
use std::{ops::Deref, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct DatabaseConnection(Arc<Mutex<Connection>>);

impl Deref for DatabaseConnection {
    type Target = Arc<Mutex<Connection>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
