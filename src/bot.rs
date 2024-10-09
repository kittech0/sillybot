use log::{error, warn};
use serenity::{
    all::{
        Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
        EventHandler, GatewayIntents, Interaction, Ready,
    },
    async_trait, Client,
};
use sillybot::read_token;

use crate::{commands::ping, ErrorResult};

pub struct BotHandler;

impl BotHandler {
    pub async fn run() -> ErrorResult {
        let token = read_token("token").await?;
        let intents = GatewayIntents::all();
        let mut client = Client::builder(&token, intents).event_handler(Self).await?;
        client.start().await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        warn!("Bot running on: {}", ready.user.name);

        let _ = Command::create_global_command(&ctx.http, ping::register()).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(ping::run(&command.data.options())),
                _ => None,
            };
            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    error!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}
