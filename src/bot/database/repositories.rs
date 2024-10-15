use chrono::NaiveDateTime;
use rusqlite::params;

use crate::util::ErrorResult;

use super::{data::User, DatabaseConnection, DatabaseHandler, UserRepository};

impl UserRepository {
    pub async fn new() -> Self {
        Self(DatabaseHandler::get_connection().await)
    }

    pub async fn init(db_conn: &DatabaseConnection) -> ErrorResult {
        db_conn.lock().await.execute(
            "CREATE TABLE IF NOT EXISTS Users (
                            discord_id BIGINT UNSIGNED UNIQUE NOT NULL,
                            join_date DATETIME NOT NULL
                        )",
            (),
        )?;
        Ok(())
    }

    pub async fn replace(&self, user: User) -> ErrorResult {
        let conn = self.0.lock().await;
        conn.execute(
            "REPLACE INTO Users (discord_id, join_date) VALUES (?1,?2)",
            params![
                user.discord_id,
                user.join_date.format("%Y-%m-%d %H:%M:%S").to_string()
            ],
        )?;
        Ok(())
    }

    pub async fn get_all(&self) -> ErrorResult<Vec<User>> {
        let conn = self.0.lock().await;
        let mut stmt = conn.prepare("SELECT discord_id, join_date FROM Users")?;
        let user_iter = stmt.query_map([], |row| {
            let string = row.get_ref(1)?.as_str()?;
            Ok(User {
                discord_id: row.get(0)?,
                join_date: NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S").unwrap(),
            })
        })?;
        Ok(user_iter.map(Result::unwrap).collect())
    }
}
