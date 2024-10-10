use std::{collections::HashMap, sync::Arc};

pub mod bothandler;
mod cmdhandler;
mod commands;
pub type CommandMap = HashMap<String, Arc<dyn commands::Command>>;

pub struct CommandHandler(CommandMap);

pub struct BotHandler {
    commands: CommandHandler,
}
