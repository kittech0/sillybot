use chrono::NaiveDateTime;
use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};

use crate::bot::database::{self, DatabaseHandler};

pub fn register(name: &str) -> CreateCommand {
    CreateCommand::new(name)
        .description("A newuser command")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "A user test").required(true),
        )
}
pub fn options(cirm: CreateInteractionResponseMessage) -> CreateInteractionResponseMessage {
    cirm.ephemeral(true)
}

pub async fn run(_ctx: &Context, options: &[ResolvedOption<'_>]) -> String {
    let Some(ResolvedOption {
        value: ResolvedValue::User(user, Some(partial)),
        ..
    }) = options.first()
    else {
        return "Provide a valid user".to_string();
    };
    let connection = DatabaseHandler::get_connection().await;
    let Some(joined_at) = partial.joined_at else {
        return "Unknown join date".to_string();
    };
    let join_date = NaiveDateTime::new(joined_at.date_naive(), joined_at.time());
    let user_db = database::User::new(user.id.get(), join_date);
    if let Err(error) = user_db.insert_or_replace(&connection).await {
        log::error!("Error: {error:?}");
        "SQL Error".to_string()
    } else {
        "Completed".to_string()
    }
}
