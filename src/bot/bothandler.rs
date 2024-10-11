use std::process::exit;

use serenity::{
    all::{Context, EventHandler, GatewayIntents, Interaction, Ready},
    async_trait, Client,
};

use crate::{
    bot::CommandHandler,
    util::{self, ErrorResult},
};

use super::BotHandler;

impl BotHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self) -> ErrorResult {
        let Some(token) = util::read_token("token").await? else {
            log::error!("Undefined token");
            exit(1)
        };
        let intents = GatewayIntents::all();
        let mut client = Client::builder(&token, intents).event_handler(self).await?;
        Ok(client.start().await?)
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::warn!("Bot running on: {}", ready.user.name);
        if let Err(error) = CommandHandler::register_global_commands(ctx).await {
            log::error!("Unable to register command: {error}")
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command_interaction) = interaction else {
            return;
        };
        if let Err(error) = CommandHandler::run_command(ctx, command_interaction).await {
            log::error!("Unable to run command: {error}")
        }
    }
}
