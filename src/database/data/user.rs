use std::{fmt::Display, str::FromStr};

use super::{DiscordId, JoinDate, SqlData, User};
use chrono::{NaiveDateTime, ParseError};
use rusqlite::Row;
use serenity::all::{Timestamp, UserId};

impl User {
    pub fn new(discord_id: DiscordId, join_date: JoinDate) -> Self {
        Self {
            discord_id,
            join_date,
        }
    }
}

impl SqlData for DiscordId {
    fn get_sql_type() -> impl AsRef<str> {
        "BIGINT UNSIGNED UNIQUE NOT NULL"
    }
}

impl SqlData for JoinDate {
    fn get_sql_type() -> impl AsRef<str> {
        "DATETIME NOT NULL"
    }
}

impl From<UserId> for DiscordId {
    fn from(value: UserId) -> Self {
        DiscordId(value.get())
    }
}

impl TryFrom<&Row<'_>> for User {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        let string = row.get_ref(1)?.as_str()?;
        let id: u64 = row.get(0)?;
        Ok(Self {
            discord_id: id.into(),
            join_date: string.parse().unwrap(),
        })
    }
}

impl From<NaiveDateTime> for JoinDate {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<Timestamp> for JoinDate {
    fn from(value: Timestamp) -> Self {
        Self(NaiveDateTime::new(value.date_naive(), value.time()))
    }
}

impl Display for DiscordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl Display for JoinDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S"))?;
        Ok(())
    }
}

impl From<u64> for DiscordId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl FromStr for JoinDate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?.into())
    }
}
