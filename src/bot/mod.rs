use commands::CommandRegistry;
use serenity::all::GuildId;

use crate::database::DatabaseConnection;

pub mod commands;
mod events;
pub mod handler;

pub struct BotHandler {
    db_conn: DatabaseConnection,
    cmd_handler: CommandHandler,
    guild_id: GuildId,
}
pub struct CommandHandler {
    registry: CommandRegistry,
}
