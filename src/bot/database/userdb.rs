use chrono::NaiveDateTime;
use rusqlite::params;

use crate::util::ErrorResult;

use super::{DatabaseConnection, User};

impl User {
    pub fn new(discord_id: u64, join_date: NaiveDateTime) -> Self {
        Self {
            discord_id,
            join_date,
        }
    }

    pub async fn init_table(conn: &DatabaseConnection) -> ErrorResult {
        conn.lock().await.execute(
            "CREATE TABLE IF NOT EXISTS Users (
                            discord_id BIGINT UNSIGNED UNIQUE NOT NULL,
                            join_date DATETIME NOT NULL
                        )",
            (),
        )?;
        Ok(())
    }

    pub async fn insert_or_replace(self, conn: &DatabaseConnection) -> ErrorResult {
        conn.lock().await.execute(
            "REPLACE INTO Users (discord_id, join_date) VALUES (?1,?2)",
            params![
                self.discord_id,
                self.join_date.format("%Y-%m-%d %H:%M:%S").to_string()
            ],
        )?;
        Ok(())
    }
}
