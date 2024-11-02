use chrono::NaiveDateTime;
use tabled::Tabled;

pub mod item;
pub mod message;
pub mod permission;
pub mod user;

#[derive(Debug, Tabled)]
pub struct UserData {
    pub discord_id: DiscordId,
}
#[derive(Debug, Tabled)]
pub struct MessageData {
    pub message_id: DiscordId,
    pub owner_id: DiscordId,
    pub message_content: MessageContent,
    pub creation_date: Date,
}

#[derive(Debug, Tabled)]
pub struct PermissionData {
    pub name: Identifier,
    pub default_value: ControlAccess,
}

#[derive(Debug, Tabled)]
pub struct ItemData {
    pub name: Identifier,
    pub item_type: ItemType,
    pub control_access: ControlAccess,
}

#[derive(Debug, Tabled)]
pub enum ItemType {
    Command,
}

#[derive(Debug, Tabled)]
pub enum ControlAccess {
    Allow,
    Disallow,
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

#[derive(Debug, Tabled)]
pub struct Identifier(String);
