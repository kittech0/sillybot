use chrono::NaiveDateTime;
use tabled::Tabled;

pub mod command;
pub mod message;
pub mod permission;
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
pub struct CommandData {
    pub row_id: usize,
    pub name: String,
}

#[derive(Debug, Tabled)]
pub struct PermissionData {
    pub name: String,
    pub default_value: PermissionValue,
}

#[derive(Debug, Tabled)]
pub enum PermissionValue {
    Allow,
    Disallow,
}

#[derive(Debug, Tabled)]
pub struct PermissionName(String);

#[derive(Debug, Tabled)]
pub struct MessageContent(String);

#[derive(Debug, Tabled)]
pub struct DiscordId(u64);
#[derive(Debug, Tabled)]
pub struct Date(NaiveDateTime);
pub trait SqlData {
    fn get_sql_type() -> impl AsRef<str>;
}
