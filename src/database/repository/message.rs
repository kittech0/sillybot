use rusqlite::params;

use crate::{
    database::{
        data::{self, SqlData},
        DatabaseConnection,
    },
    util::{self, ErrorResult},
};

use super::MessageRepository;

impl MessageRepository {
    pub fn get(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    pub async fn init(db_conn: DatabaseConnection) -> ErrorResult {
        db_conn.lock().await.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS Messages (
                    message_id {} UNIQUE PRIMARY KEY,
                    owner_id {},
                    message_content  {},
                    creation_date {},
                    FOREIGN KEY(owner_id) REFERENCES Users(user_id)
                ) STRICT",
                data::DiscordId::get_sql_type().as_ref(),
                data::DiscordId::get_sql_type().as_ref(),
                data::MessageContent::get_sql_type().as_ref(),
                data::Date::get_sql_type().as_ref(),
            ),
            (),
        )?;
        log::warn!("loading messages repository");
        Ok(())
    }
    pub async fn replace(&self, message: data::MessageData) -> ErrorResult {
        let conn = self.db_conn.lock().await;
        conn.execute(
            "REPLACE INTO Messages (message_id, owner_id, message_content, creation_date) VALUES (?1,?2,?3,?4)",
            params![
                message.message_id.to_string(), 
                message.owner_id.to_string(), 
                message.message_content.to_string(), 
                message.creation_date.to_string()
            ],
        )?;
        Ok(())
    }

    pub async fn get_all(&self) -> ErrorResult<Vec<data::MessageData>> {
        let conn = self.db_conn.lock().await;
        let mut stmt = conn.prepare("SELECT message_id, owner_id, message_content, creation_date FROM Messages")?;
        let user_iter = stmt.query_map([], |r| data::MessageData::try_from(r))?;
        user_iter.map(|r| r.map_err(util::Error::from)).collect()
    }
}
