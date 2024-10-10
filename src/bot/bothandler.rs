use serenity::{
    all::{
        Command, CommandInteraction, Context, CreateInteractionResponse, EventHandler,
        GatewayIntents, Interaction, Ready,
    },
    async_trait, Client,
};

use crate::util::{self, ErrorResult};

use super::{BotHandler, CommandHandler};

impl BotHandler {
    pub fn new() -> Self {
        Self {
            commands: CommandHandler::new(),
        }
    }

    pub async fn run(self) -> ErrorResult {
        let token = util::read_token("token").await?;
        let intents = GatewayIntents::all();
        let mut client = Client::builder(&token, intents).event_handler(self).await?;
        Ok(client.start().await?)
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::warn!("Bot running on: {}", ready.user.name);
        if let Err(error) = self.commands.register_global_commands(ctx).await {
            log::error!("Unable to register command: {error}")
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command_interaction) = interaction {
            if let Err(error) = self.commands.run_commands(ctx, command_interaction).await {
                log::error!("Unable to run command: {error}")
            }
        }
    }
}
