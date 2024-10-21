use chrono::NaiveDateTime;
use tabled::Tabled;

pub mod messages;
pub mod user;

#[derive(Debug, Tabled)]
pub struct UserData {
    pub discord_id: DiscordId,
    pub join_date: Date,
}
#[derive(Debug, Tabled)]
pub struct MessageData {
    pub message_id: DiscordId,
    pub owner_id: DiscordId,
    pub message_content: MessageContent,
    pub creation_date: Date,
}

#[derive(Debug, Tabled)]
pub struct MessageContent(String);

#[derive(Debug, Tabled)]
pub struct DiscordId(u64);
#[derive(Debug, Tabled)]
pub struct Date(NaiveDateTime);
pub trait SqlData {
    fn get_sql_type() -> impl AsRef<str>;
}
