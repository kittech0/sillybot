use std::sync::Arc;

use serenity::all::{Command, CommandInteraction, Context, CreateInteractionResponse};

use crate::util::ErrorResult;

use super::{commands, CommandHandler, CommandMap};

impl CommandHandler {
    pub fn new() -> Self {
        let mut cmd_map = CommandMap::new();
        cmd_map.insert("ping".to_string(), Arc::new(commands::PingCommand));
        cmd_map.insert(
            "testinput".to_string(),
            Arc::new(commands::TestInputCommand),
        );
        Self(cmd_map)
    }

    pub async fn register_global_commands(&self, ctx: Context) -> ErrorResult {
        for (k, v) in &self.0 {
            Command::create_global_command(&ctx.http, v.register().await).await?;
            log::warn!("Loading slash command: {k}")
        }
        Ok(())
    }

    pub async fn run_commands(
        &self,
        ctx: Context,
        command_interaction: CommandInteraction,
    ) -> ErrorResult {
        if let Some(content) = self.0.get(&command_interaction.data.name) {
            let data = content
                .options()
                .await
                .content(content.run(&command_interaction.data.options()).await);
            let builder = CreateInteractionResponse::Message(data);
            command_interaction
                .create_response(&ctx.http, builder)
                .await?;
        }
        Ok(())
    }
}
