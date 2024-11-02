use commands::CommandRegistry;
use serenity::all::GuildId;

use crate::database::DatabaseConnection;

pub mod commands;
mod events;
pub mod handler;

pub struct BotHandler {
    event_manager: EventManager,
}
pub struct CommandHandler {
    command_registry: CommandRegistry,
}

pub struct EventManager {
    db_conn: DatabaseConnection,
    cmd_handler: CommandHandler,
}
