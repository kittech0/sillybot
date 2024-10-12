use std::sync::Arc;

use rusqlite::Connection;
use tokio::sync::{Mutex, OnceCell};

use super::DatabaseHandler;

impl DatabaseHandler {
    pub async fn get_connection() -> Self {
        static CONNECTION: OnceCell<Arc<Mutex<Connection>>> = OnceCell::const_new();
        DatabaseHandler(
            CONNECTION
                .get_or_init(|| async {
                    Arc::new(Mutex::new(Connection::open_in_memory().unwrap()))
                })
                .await
                .clone(),
        )
    }
}

impl Clone for DatabaseHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
