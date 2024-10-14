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
pub async fn run(
    _ctx: &Context,
    options: &[ResolvedOption<'_>],
    cirm: CreateInteractionResponseMessage,
) -> CreateInteractionResponseMessage {
    let cirm = cirm.ephemeral(true);
    let Some(ResolvedOption {
        value: ResolvedValue::User(user, Some(partial)),
        ..
    }) = options.first()
    else {
        return cirm.content("Provide a valid user");
    };
    let connection = DatabaseHandler::get_connection().await;
    let Some(joined_at) = partial.joined_at else {
        return cirm.content("Unknown join date");
    };
    let join_date = NaiveDateTime::new(joined_at.date_naive(), joined_at.time());
    let user_db = database::User::new(user.id.get(), join_date);
    if let Err(error) = user_db.insert_or_replace(&connection).await {
        log::error!("Error: {error:?}");
        cirm.content("SQL Error")
    } else {
        cirm.content("Completed")
    }
}
