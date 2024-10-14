use std::str::FromStr;

use serenity::all::{
    Command, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, GuildId,
};
use strum::IntoEnumIterator;

use crate::util::ErrorResult;

use super::{commands, CommandHandler};

impl CommandHandler {
    pub async fn _register_global_commands(ctx: Context) -> ErrorResult {
        for v in commands::Command::iter() {
            Command::create_global_command(&ctx.http, v.register()).await?;
            let name: &'static str = v.into();
            log::warn!("Loaded global slash command: {name}")
        }
        Ok(())
    }

    pub async fn register_guild_commands(ctx: Context, guild_id: GuildId) -> ErrorResult {
        guild_id
            .set_commands(
                &ctx.http,
                commands::Command::iter().map(|x| x.register()).collect(),
            )
            .await?;
        for command in commands::Command::iter() {
            let name: &'static str = command.into();
            log::warn!("Loaded guild ({guild_id}) slash command: {name}");
        }
        Ok(())
    }

    pub async fn run_command(ctx: Context, command_interaction: CommandInteraction) -> ErrorResult {
        let Ok(content) = commands::Command::from_str(&command_interaction.data.name) else {
            return Ok(());
        };
        let data = CreateInteractionResponseMessage::new()
            .content(content.run(&ctx, &command_interaction.data.options()).await);
        command_interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(content.options(data)),
            )
            .await?;
        Ok(())
    }
}
