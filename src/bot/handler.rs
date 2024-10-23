use serenity::{
    all::{Context, EventHandler, GatewayIntents, GuildId, Interaction, Message, Ready},
    async_trait, Client,
};

use crate::{
    bot::CommandHandler,
    database::{data::MessageData, repository::MessagesRepository, DatabaseConnection},
    util::{funcs, Error, ErrorResult},
};

use super::{events, BotHandler};

impl BotHandler {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self {
            db_conn: db_conn.clone(),
            cmd_handler: CommandHandler::new(db_conn),
            guild_id: GuildId::new(1285696315640123553),
        }
    }

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
        events::on_ready(self, ctx, ready).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::on_interaction_create(self, ctx, interaction).await;
    }

    async fn message(&self, ctx: Context, message: Message) {
        events::on_message(self, ctx, message).await;
    }
}
