use serenity::{
    all::{CreateCommand, CreateInteractionResponseMessage, ResolvedOption},
    async_trait,
};

use super::{CommandRunner, PingCommand};

impl PingCommand {
    pub fn register() -> CreateCommand {
        CreateCommand::new("ping").description("A ping command")
    }

    pub fn options() -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new().ephemeral(true)
    }
}

#[async_trait]
impl CommandRunner for PingCommand {
    async fn run(&self, _options: &[ResolvedOption]) -> String {
        "Hey, I'm alive!".to_string()
    }
}
