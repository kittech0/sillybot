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
    pub fn get(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }

    pub async fn init(db_conn: DatabaseConnection) -> ErrorResult {
        db_conn.lock().await.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS Users (
                    user_id {} UNIQUE,
                    join_date {}
                ) STRICT",
                data::DiscordId::get_sql_type().as_ref(),
                data::Date::get_sql_type().as_ref()
            ),
            (),
        )?;
        log::warn!("loading users repository");

        Ok(())
    }

    pub async fn replace(&self, user: data::UserData) -> ErrorResult {
        let conn = self.db_conn.lock().await;
        conn.execute(
            "REPLACE INTO Users (user_id, join_date) VALUES (?1,?2)",
            params![user.discord_id.to_string(), user.join_date.to_string()],
        )?;
        Ok(())
    }

    pub async fn get_all(&self) -> ErrorResult<Vec<data::UserData>> {
        let conn = self.db_conn.lock().await;
        let mut stmt = conn.prepare("SELECT user_id, join_date FROM Users")?;
        let user_iter = stmt.query_map([], |r| data::UserData::try_from(r))?;
        user_iter.map(|r| r.map_err(util::Error::from)).collect()
    }
}
