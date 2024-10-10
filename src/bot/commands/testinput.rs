use serenity::{
    all::{
        CommandOptionType, CreateCommand, CreateCommandOption, CreateInteractionResponseMessage,
        ResolvedOption, ResolvedValue,
    },
    async_trait,
};

use super::{Command, TestInputCommand};

#[async_trait]
impl Command for TestInputCommand {
    async fn run(&self, options: &[ResolvedOption]) -> String {
        if let Some(ResolvedOption {
            value: ResolvedValue::String(text),
            ..
        }) = options.first()
        {
            text.to_string()
        } else {
            "Please provide a valid text".to_string()
        }
    }

    async fn register(&self) -> CreateCommand {
        CreateCommand::new("testinput")
            .description("A testinput command")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "text", "A text")
                    .required(true),
            )
    }

    async fn options(&self) -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new().ephemeral(true)
    }
}
