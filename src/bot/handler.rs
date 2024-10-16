use serenity::{
    all::{Context, EventHandler, GatewayIntents, GuildId, Interaction, Ready},
    async_trait, Client,
};

use crate::{
    bot::CommandHandler,
    util::{funcs, Error, ErrorResult},
};

use super::BotHandler;

impl BotHandler {
    pub async fn run(self) -> ErrorResult {
        let token = funcs::read_token("token")?.ok_or(Error::UndefinedToken)?;
        let intents = GatewayIntents::all();
        let mut client = Client::builder(&token, intents).event_handler(self).await?;

        Ok(client.start().await?)
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::warn!("bot running on: {}", ready.user.name);
        let guild_id = GuildId::new(1285696315640123553);
        if let Err(error) = CommandHandler.register_guild_commands(ctx, guild_id).await {
            funcs::throw("unable to register command", error);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command_interaction) = interaction else {
            return;
        };
        if let Err(error) = CommandHandler.run_command(ctx, command_interaction).await {
            funcs::throw("unable to run command", error);
        }
    }
}
