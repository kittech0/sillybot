use std::sync::Arc;

use tokio::sync::OnceCell;
use tokio_rusqlite::Connection;

use super::DatabaseHandler;

impl DatabaseHandler {
    pub async fn get_connection() -> Self {
        static CONNECTION: OnceCell<Arc<Connection>> = OnceCell::const_new();
        DatabaseHandler(
            CONNECTION
                .get_or_init(|| async { Arc::new(Connection::open_in_memory().await.unwrap()) })
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
