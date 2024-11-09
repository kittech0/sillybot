use serenity::{
    all::{Context, EventHandler, GuildId, Interaction, Message, Ready},
    async_trait,
};

use crate::{
    database::{data::MessageData, repository::MessageRepository, DatabaseConnection},
    util::funcs,
    GUILD_ID,
};

use super::{CommandHandler, EventManager};

impl EventManager {
    pub fn new(db_conn: DatabaseConnection, cmd_handler: CommandHandler) -> Self {
        Self {
            db_conn,
            cmd_handler,
        }
    }
}

#[async_trait]
impl EventHandler for EventManager {
    async fn message(&self, _ctx: Context, message: Message) {
        if message.author.bot && message.content.is_empty() {
            return;
        }
        let Some(_member) = message.member else {
            return;
        };

        let message_id = message.id;
        let owner_id = message.author.id;
        let message_content = message.content;
        let creation_date = message.timestamp;

        let message_data = MessageData::new(
            message_id.into(),
            owner_id.into(),
            message_content.into(),
            creation_date.into(),
        );
        let repository = MessageRepository::get(self.db_conn.clone());
        if let Err(error) = repository.replace(message_data).await {
            log::error!("sql error: {error:?}")
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::warn!("bot running on: {}", ready.user.name);
        if let Err(error) = self
            .cmd_handler
            .register_guild_commands(ctx, GuildId::new(GUILD_ID))
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
}
