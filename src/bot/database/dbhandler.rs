use std::sync::Arc;

use tokio::sync::OnceCell;
use tokio_rusqlite::Connection;

use crate::util::ErrorResult;

use super::DatabaseHandler;

static CONNECTION: OnceCell<Arc<Connection>> = OnceCell::const_new();

impl DatabaseHandler {
    pub async fn get_connection() -> Self {
        DatabaseHandler {
            connection: CONNECTION
                .get_or_init(|| async { Arc::new(Connection::open_in_memory().await.unwrap()) })
                .await
                .clone(),
        }
    }
}

impl Clone for DatabaseHandler {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
        }
    }
}
