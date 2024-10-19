
use commands::CommandRegistry;

pub mod commands;
pub mod handler;

pub struct BotHandler {
    cmd_handler: CommandHandler,
}
pub struct CommandHandler {
    registry: CommandRegistry,
}
