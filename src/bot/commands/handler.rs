use std::str::FromStr;

use serenity::all::{
    Command, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, GuildId,
};
use strum::IntoEnumIterator;

use crate::{bot::CommandHandler, util::ErrorResult};

use super::CommandStore;

impl CommandHandler {
    pub async fn _register_global_commands(&self, ctx: Context) -> ErrorResult {
        for v in CommandStore::iter() {
            Command::create_global_command(&ctx.http, v.register()).await?;
            let name: &'static str = v.into();
            log::warn!("Loaded global slash command: {name}")
        }
        Ok(())
    }

    pub async fn register_guild_commands(&self, ctx: Context, guild_id: GuildId) -> ErrorResult {
        guild_id
            .set_commands(
                &ctx.http,
                CommandStore::iter().map(|x| x.register()).collect(),
            )
            .await?;
        for command in CommandStore::iter() {
            let name: &'static str = command.into();
            log::warn!("loaded guild ({guild_id}) slash command: {name}");
        }
        Ok(())
    }

    pub async fn run_command(
        &self,
        ctx: Context,
        command_interaction: CommandInteraction,
    ) -> ErrorResult {
        let Ok(content) = CommandStore::from_str(&command_interaction.data.name) else {
            return Ok(());
        };
        let data = CreateInteractionResponseMessage::new();
        let data = content
            .run(&ctx, &command_interaction.data.options(), data)
            .await;
        command_interaction
            .create_response(&ctx.http, CreateInteractionResponse::Message(data))
            .await?;
        Ok(())
    }
}
