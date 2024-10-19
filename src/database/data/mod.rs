use chrono::NaiveDateTime;
use tabled::Tabled;

pub mod user;

#[derive(Debug, Tabled)]
pub struct User {
    pub discord_id: DiscordId,
    pub join_date: JoinDate,
}
#[derive(Debug, Tabled)]
pub struct DiscordId(u64);
#[derive(Debug, Tabled)]
pub struct JoinDate(NaiveDateTime);
pub trait SqlData {
    fn get_sql_type() -> impl AsRef<str>;
}
