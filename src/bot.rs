use log::{error, warn};
use serenity::{
    all::{
        Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
        EventHandler, GatewayIntents, Interaction, Message, Ready,
    },
    async_trait, Client,
};
use sillybot::read_config;

use crate::{commands, ErrorResult};

pub struct BotHandler;

impl BotHandler {
    pub async fn run() -> ErrorResult {
        let config = read_config("config.toml").await?;
        let intents = GatewayIntents::all();
        let mut client = Client::builder(&config.token, intents)
            .event_handler(Self)
            .await?;
        client.start().await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    error!("Error sending message: {why:?}");
                }
            }
            _ => return,
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        warn!("Bot running on: {}", ready.user.name);

        let _ = Command::create_global_command(&ctx.http, commands::register()).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::run(&command.data.options())),
                _ => None,
            };
            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    error!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}
