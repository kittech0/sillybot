use rusqlite::params;

use crate::{
    database::{
        data::{self, SqlData},
        DatabaseConnection,
    },
    util::{self, ErrorResult},
};

use super::UserRepository;

impl UserRepository {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }

    pub async fn init(db_conn: DatabaseConnection) -> ErrorResult {
        db_conn.0.lock().await.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS Users (
                    discord_id {},
                    join_date  {}
                )",
                data::DiscordId::get_sql_type().as_ref(),
                data::JoinDate::get_sql_type().as_ref()
            ),
            (),
        )?;
        Ok(())
    }

    pub async fn replace(&self, user: data::User) -> ErrorResult {
        let conn = self.db_conn.0.lock().await;
        conn.execute(
            "REPLACE INTO Users (discord_id, join_date) VALUES (?1,?2)",
            params![user.discord_id.to_string(), user.join_date.to_string()],
        )?;
        Ok(())
    }

    pub async fn get_all(&self) -> ErrorResult<Vec<data::User>> {
        let conn = self.db_conn.0.lock().await;
        let mut stmt = conn.prepare("SELECT discord_id, join_date FROM Users")?;
        let user_iter = stmt.query_map([], |r| data::User::try_from(r))?;
        user_iter.map(|r| r.map_err(util::Error::from)).collect()
    }
}
