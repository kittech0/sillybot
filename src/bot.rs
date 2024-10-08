use serenity::{
    all::{Context, EventHandler, GatewayIntents, Message},
    async_trait, Client,
};

use crate::{read_config, ErrorResult};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

pub async fn run_bot() -> ErrorResult {
    let config = read_config("config.toml").await?;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    client.start().await?;
    Ok(())
}
