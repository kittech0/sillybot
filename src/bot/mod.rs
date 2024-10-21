use commands::CommandRegistry;

use crate::database::DatabaseConnection;

pub mod commands;
pub mod handler;

pub struct BotHandler {
    db_conn: DatabaseConnection,
    cmd_handler: CommandHandler,
}
pub struct CommandHandler {
    registry: CommandRegistry,
}
