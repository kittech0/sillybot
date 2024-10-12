use serenity::{
    all::{CreateCommand, CreateInteractionResponseMessage, ResolvedOption},
    async_trait,
};

use super::{CommandRegister, CommandRunner, PingCommand};

impl CommandRegister for PingCommand {
    fn register(name: &str) -> CreateCommand {
        CreateCommand::new(name).description("A ping command")
    }

    fn options() -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new().ephemeral(true)
    }
}

#[async_trait]
impl CommandRunner for PingCommand {
    async fn run(&self, _options: &[ResolvedOption]) -> String {
        "Hey, I'm alive!".to_string()
    }
}
