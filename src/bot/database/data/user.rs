use super::User;
use chrono::NaiveDateTime;

impl User {
    pub fn new(discord_id: u64, join_date: NaiveDateTime) -> Self {
        Self {
            discord_id,
            join_date,
        }
    }
}
