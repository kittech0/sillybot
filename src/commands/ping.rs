use serenity::{
    all::{CreateCommand, CreateInteractionResponseMessage, ResolvedOption},
    async_trait,
};

use super::Command;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    async fn run(&self, _options: &[ResolvedOption]) -> String {
        "Hey, I'm alive!".to_string()
    }

    async fn register(&self) -> CreateCommand {
        CreateCommand::new("ping").description("A ping command")
    }

    async fn options(&self) -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new().ephemeral(true)
    }
}
