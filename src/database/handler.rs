use std::{
    path::PathBuf,
    sync::Arc,
};

use rusqlite::Connection;
use tokio::sync::Mutex;

use crate::util::ErrorResult;

use super::{DatabaseConnection, DatabaseHandler};

impl DatabaseHandler {
    pub fn new(connection_path: Option<PathBuf>) -> ErrorResult<Self> {
        let connection = match connection_path {
            Some(path) => Connection::open(path)?,
            None => Connection::open_in_memory()?,
        };
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}

impl Clone for DatabaseHandler {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
        }
    }
}
