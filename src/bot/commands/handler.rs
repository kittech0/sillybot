use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    GuildId,
};

use crate::{bot::CommandHandler, util::ErrorResult};

use super::CommandRegistry;

impl CommandHandler {
    pub fn new(command_registry: CommandRegistry) -> Self {
        Self { command_registry }
    }

    pub async fn register_guild_commands(&self, ctx: Context, guild_id: GuildId) -> ErrorResult {
        let registry = &self.command_registry.0;
        guild_id
            .set_commands(
                &ctx.http,
                registry
                    .iter()
                    .map(|(name, cmd)| cmd.register(name))
                    .collect(),
            )
            .await?;
        for name in registry.keys() {
            log::warn!("loaded guild ({guild_id}) slash command: {name}");
        }
        Ok(())
    }

    pub async fn run_command(
        &self,
        ctx: Context,
        command_interaction: CommandInteraction,
    ) -> ErrorResult {
        let registry = &self.command_registry;
        let data = CreateInteractionResponseMessage::new();
        let Some(data) = registry
            .run(
                &command_interaction.data.name,
                &ctx,
                &command_interaction.data.options(),
                data,
            )
            .await
        else {
            return Ok(());
        };
        command_interaction
            .create_response(&ctx.http, CreateInteractionResponse::Message(data))
            .await?;
        Ok(())
    }
}
