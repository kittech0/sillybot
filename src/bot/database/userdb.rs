use chrono::NaiveDateTime;
use rusqlite::params;

use crate::util::ErrorResult;

use super::{DatabaseHandler, User};

impl User {
    pub fn new(discord_id: u64, join_date: NaiveDateTime) -> Self {
        Self {
            discord_id,
            join_date,
        }
    }
    pub async fn new_table(db_handler: &DatabaseHandler) -> ErrorResult {
        let connection_fn = move |conn: &mut rusqlite::Connection| {
            Ok(conn.execute(
                "CREATE TABLE IF NOT EXISTS Users (
                            discord_id BIGINT UNSIGNED UNIQUE NOT NULL,
                            join_date DATETIME NOT NULL
                        )",
                (),
            )?)
        };
        db_handler.connection.call(connection_fn).await?;
        Ok(())
    }

    pub async fn insert(self, db_handler: &DatabaseHandler) -> ErrorResult {
        let connection_fn = move |conn: &mut rusqlite::Connection| {
            Ok(conn.execute(
                "INSERT INTO Users (discord_id, join_date) VALUES (?1,?2)",
                params![
                    self.discord_id,
                    self.join_date.format("%Y-%m-%d %H:%M:%S").to_string()
                ],
            )?)
        };
        db_handler.connection.call(connection_fn).await?;
        Ok(())
    }
}
