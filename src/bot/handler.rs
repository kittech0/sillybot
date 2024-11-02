use serenity::{
    all::{Context, EventHandler, GatewayIntents, GuildId, Interaction, Message, Ready},
    async_trait, Client,
};

use crate::{
    bot::CommandHandler,
    database::DatabaseConnection,
    util::{funcs, Error, ErrorResult},
};

use super::{commands::CommandRegistry, events, BotHandler, EventManager};

impl BotHandler {
    pub fn new(event_manager: EventManager) -> Self {
        Self {
            event_manager,
        }
    }

    pub async fn run(self) -> ErrorResult {
        let token = funcs::read_token("token")?.ok_or(Error::UndefinedToken)?;
        let intents = GatewayIntents::all();
        let mut client = Client::builder(&token, intents)
            .event_handler(self.event_manager)
            .await?;

        Ok(client.start().await?)
    }
}
