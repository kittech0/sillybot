use std::fmt::Display;

use rusqlite::Row;

use super::{Date, DiscordId, MessageContent, MessageData, SqlData};

impl MessageData {
    pub fn new(
        message_id: DiscordId,
        owner_id: DiscordId,
        message_content: MessageContent,
        creation_date: Date,
    ) -> Self {
        Self {
            message_id,
            owner_id,
            message_content,
            creation_date,
        }
    }
}

impl SqlData for MessageContent {
    fn get_sql_type() -> impl AsRef<str> {
        "TEXT NOT NULL"
    }
}
impl TryFrom<&Row<'_>> for MessageData {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        let message_id: u64 = row.get(0)?;
        let owner_id: u64 = row.get(1)?;
        let message_content = row.get_ref(2)?.as_str()?;
        let creation_date = row.get_ref(3)?.as_str()?;

        Ok(Self {
            message_id: message_id.into(),
            owner_id: owner_id.into(),
            message_content: message_content.into(),
            creation_date: creation_date.parse().unwrap(),
        })
    }
}

impl<T: Into<String>> From<T> for MessageContent {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Display for MessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
