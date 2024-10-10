use std::{collections::HashMap, sync::Arc};

use log::{error, warn};
use serenity::{
    all::{
        Command, CommandInteraction, Context, CreateInteractionResponse, EventHandler,
        GatewayIntents, Interaction, Ready,
    },
    async_trait, Client,
};
use sillybot::read_token;

use crate::{
    commands::{self, ping::PingCommand, testinput::TestInputCommand},
    ErrorResult,
};

pub struct BotHandler {
    commands: HashMap<String, Arc<dyn commands::Command>>,
}

impl BotHandler {
    pub async fn new() -> Self {
        let mut new = Self {
            commands: HashMap::new(),
        };
        new.commands
            .insert("ping".to_string(), Arc::new(PingCommand));
        new.commands
            .insert("testinput".to_string(), Arc::new(TestInputCommand));
        new
    }

    pub async fn run(self) -> ErrorResult {
        let token = read_token("token").await?;
        let intents = GatewayIntents::all();
        let mut client = Client::builder(&token, intents).event_handler(self).await?;
        client.start().await?;
        Ok(())
    }

    pub async fn register_global_commands(&self, ctx: Context) {
        for (k, v) in &self.commands {
            Command::create_global_command(&ctx.http, v.register().await)
                .await
                .unwrap();
            warn!("Loading slash command: {k}")
        }
    }

    pub async fn run_commands(&self, ctx: Context, command_interaction: CommandInteraction) {
        if let Some(content) = self.commands.get(&command_interaction.data.name) {
            let data = content
                .options()
                .await
                .content(content.run(&command_interaction.data.options()).await);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command_interaction
                .create_response(&ctx.http, builder)
                .await
            {
                error!("Cannot respond to slash command: {why}");
            }
        }
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        warn!("Bot running on: {}", ready.user.name);
        self.register_global_commands(ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command_interaction) = interaction {
            self.run_commands(ctx, command_interaction).await;
        }
    }
}
