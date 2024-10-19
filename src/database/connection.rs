use std::{path::PathBuf, sync::Arc};

use rusqlite::Connection;
use tokio::sync::Mutex;

use crate::util::ErrorResult;

use super::DatabaseConnection;

impl DatabaseConnection {
    pub fn new(connection_path: Option<PathBuf>) -> ErrorResult<Self> {
        let connection = match connection_path {
            Some(path) => Connection::open(path)?,
            None => Connection::open_in_memory()?,
        };
        Ok(Self(Arc::new(Mutex::new(connection))))
    }
}

impl Clone for DatabaseConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
