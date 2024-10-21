use serenity::{
    all::{Context, EventHandler, GatewayIntents, GuildId, Interaction, Message, Ready},
    async_trait, Client,
};

use crate::{
    bot::CommandHandler,
    database::{data::MessageData, repository::MessagesRepository, DatabaseConnection},
    util::{funcs, Error, ErrorResult},
};

use super::BotHandler;

impl BotHandler {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self {
            db_conn: db_conn.clone(),
            cmd_handler: CommandHandler::new(db_conn),
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
        log::warn!("bot running on: {}", ready.user.name);
        let guild_id = GuildId::new(1285696315640123553);
        if let Err(error) = self
            .cmd_handler
            .register_guild_commands(ctx, guild_id)
            .await
        {
            funcs::throw("unable to register command", error);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command_interaction) = interaction else {
            return;
        };
        if let Some(error) = self
            .cmd_handler
            .run_command(ctx, command_interaction)
            .await
            .err()
        {
            funcs::throw("unable to run command", error);
        }
    }

    async fn message(&self, _ctx: Context, message: Message) {
        if message.author.bot && message.content.is_empty() {
            return;
        }
        let Some(_member) = message.member else {
            return;
        };

        let message_id = message.id.get();
        let owner_id = message.author.id;
        let message_content = message.content;
        let creation_date = message.timestamp;

        let message_data = MessageData::new(
            message_id.into(),
            owner_id.into(),
            message_content.into(),
            creation_date.into(),
        );
        let repository = MessagesRepository::new(self.db_conn.clone());
        if let Err(error) = repository.replace(message_data).await {
            log::error!("sql error: {error:?}")
        }
    }
}
