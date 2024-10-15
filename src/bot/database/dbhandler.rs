use std::sync::Arc;

use rusqlite::Connection;
use tokio::sync::{Mutex, OnceCell};

use super::{DatabaseConnection, DatabaseHandler, UserRepository};

impl DatabaseHandler {
    pub async fn get_connection() -> DatabaseConnection {
        static CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();
        CONNECTION
            .get_or_init(Self::init_repositories)
            .await
            .clone()
    }

    async fn init_repositories() -> DatabaseConnection {
        let db = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        UserRepository::init(&db).await.unwrap();
        db
    }
}
