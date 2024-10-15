use chrono::NaiveDateTime;
use tabled::Tabled;

pub mod user;

#[derive(Debug, Tabled)]
pub struct User {
    pub discord_id: u64,
    pub join_date: NaiveDateTime,
}
