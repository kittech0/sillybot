use std::{collections::HashMap, str::FromStr};

use serenity::all::{
    Command, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, GuildId,
};

use crate::{bot::CommandHandler, database::DatabaseConnection, util::ErrorResult};

use super::{CommandRegistry, NewUserCmd, PingCmd};

impl CommandHandler {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        let mut registry = CommandRegistry::new();
        registry.add("newuser", Box::new(NewUserCmd::new(db_conn.clone())));
        registry.add("ping", Box::new(PingCmd {}));

        Self { registry }
    }
    // pub async fn _register_global_commands(&self, ctx: Context) -> ErrorResult {
    //     for (name, cmd) in &self.registry.0 {
    //         Command::create_global_command(&ctx.http, cmd.register(name)).await?;
    //         log::warn!("loaded global slash command: {name}")
    //     }
    //     Ok(())
    // }

    pub async fn register_guild_commands(&self, ctx: Context, guild_id: GuildId) -> ErrorResult {
        let registry = &self.registry.0;
        guild_id
            .set_commands(
                &ctx.http,
                registry
                    .iter()
                    .map(|(name, cmd)| cmd.register(&name))
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
        let registry = &self.registry.0;

        let Some(content) = registry.get(&command_interaction.data.name) else {
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
