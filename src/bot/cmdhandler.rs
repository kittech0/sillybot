use std::str::FromStr;

use serenity::all::{Command, CommandInteraction, Context, CreateInteractionResponse};
use strum::IntoEnumIterator;

use crate::util::ErrorResult;

use super::{commands, CommandHandler};

impl CommandHandler {
    pub async fn register_global_commands(ctx: Context) -> ErrorResult {
        for v in commands::Command::iter() {
            Command::create_global_command(&ctx.http, v.register()).await?;
            let name: &'static str = v.into();
            log::warn!("Loading slash command: {name}")
        }
        Ok(())
    }

    pub async fn run_command(ctx: Context, command_interaction: CommandInteraction) -> ErrorResult {
        let Ok(content) = commands::Command::from_str(&command_interaction.data.name) else {
            return Ok(());
        };
        let data = content.options().content(
            content
                .runner()
                .run(&command_interaction.data.options())
                .await,
        );
        command_interaction
            .create_response(&ctx.http, CreateInteractionResponse::Message(data))
            .await?;
        Ok(())
    }
}
