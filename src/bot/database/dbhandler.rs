use std::sync::Arc;

use rusqlite::Connection;
use tokio::sync::{Mutex, OnceCell};

use super::{DatabaseConnection, DatabaseHandler, User};

impl DatabaseHandler {
    pub async fn get_connection() -> DatabaseConnection {
        static CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();
        CONNECTION.get_or_init(Self::init).await.clone()
    }

    async fn init() -> DatabaseConnection {
        let db = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        User::init_table(&db).await.unwrap();
        db
    }
}
