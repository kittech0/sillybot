use chrono::NaiveDateTime;
use serenity::{
    all::{
        CommandOptionType, CreateCommand, CreateCommandOption, CreateInteractionResponseMessage,
        ResolvedOption, ResolvedValue,
    },
    async_trait,
};

use crate::bot::database::{self, DatabaseHandler};

use super::{CommandRunner, TestInputCommand};

impl TestInputCommand {
    pub fn register() -> CreateCommand {
        CreateCommand::new("testinput")
            .description("A testinput command")
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "user", "A user test")
                    .required(true),
            )
    }

    pub fn options() -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new().ephemeral(true)
    }
}

#[async_trait]
impl CommandRunner for TestInputCommand {
    async fn run(&self, options: &[ResolvedOption]) -> String {
        let Some(ResolvedOption {
            value: ResolvedValue::User(user, Some(partial)),
            ..
        }) = options.first()
        else {
            return "Provide a valid user".to_string();
        };
        let connection = DatabaseHandler::get_connection().await;
        if let Err(error) = database::User::new_table(&connection).await {
            log::error!("Error: {error:?}");
            return "SQL Error".to_string();
        }
        let Some(joined_at) = partial.joined_at else {
            return "Unknown join date".to_string();
        };
        let join_date = NaiveDateTime::new(joined_at.date_naive(), joined_at.time());
        let user_db = database::User::new(user.id.get(), join_date);
        if let Err(error) = user_db.insert(&connection).await {
            log::error!("Error: {error:?}");
            "SQL Error".to_string()
        } else {
            "Completed".to_string()
        }
    }
}
