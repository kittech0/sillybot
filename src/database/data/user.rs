use std::{fmt::Display, str::FromStr};

use super::{Date, DiscordId, SqlData, UserData};
use chrono::{NaiveDateTime, ParseError};
use rusqlite::Row;
use serenity::all::{MessageId, Timestamp, UserId};

impl UserData {
    pub fn new(discord_id: DiscordId) -> Self {
        Self {
            discord_id,
        }
    }
}

impl TryFrom<&Row<'_>> for UserData {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        let discord_id: u64 = row.get(0)?;
        Ok(Self {
            discord_id: discord_id.into(),
        })
    }
}
impl SqlData for DiscordId {
    fn get_sql_type() -> impl AsRef<str> {
        "INTEGER NOT NULL"
    }
}

impl SqlData for Date {
    fn get_sql_type() -> impl AsRef<str> {
        "TEXT NOT NULL"
    }
}

impl From<UserId> for DiscordId {
    fn from(value: UserId) -> Self {
        Self(value.get())
    }
}

impl From<MessageId> for DiscordId {
    fn from(value: MessageId) -> Self {
        Self(value.get())
    }
}

impl From<Timestamp> for Date {
    fn from(value: Timestamp) -> Self {
        Self(NaiveDateTime::new(value.date_naive(), value.time()))
    }
}

impl Display for DiscordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl From<u64> for DiscordId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?))
    }
}
