use serenity::all::{Context, Interaction, Message, Ready};

use crate::{
    database::{data::MessageData, repository::MessagesRepository},
    util::funcs,
};

use super::BotHandler;

pub async fn on_ready(bot_handler: &BotHandler, ctx: Context, ready: Ready) {
    log::warn!("bot running on: {}", ready.user.name);
    if let Err(error) = bot_handler
        .cmd_handler
        .register_guild_commands(ctx, bot_handler.guild_id)
        .await
    {
        funcs::throw("unable to register command", error);
    }
}

pub async fn on_interaction_create(
    bot_handler: &BotHandler,
    ctx: Context,
    interaction: Interaction,
) {
    let Interaction::Command(command_interaction) = interaction else {
        return;
    };
    if let Some(error) = bot_handler
        .cmd_handler
        .run_command(ctx, command_interaction)
        .await
        .err()
    {
        funcs::throw("unable to run command", error);
    }
}

pub async fn on_message(bot_handler: &BotHandler, _ctx: Context, message: Message) {
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
    let repository = MessagesRepository::get(bot_handler.db_conn.clone());
    if let Err(error) = repository.replace(message_data).await {
        log::error!("sql error: {error:?}")
    }
}
