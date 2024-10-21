use serenity::{
    all::{
        CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
    },
    async_trait,
};
use tabled::Table;

use crate::{
    database::{data, repository::UserRepository, DatabaseConnection},
    util::funcs,
};

use super::{Command, NewUserCmd};
impl NewUserCmd {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    async fn replace(
        &self,
        _ctx: &Context,
        options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage {
        let Some(ResolvedOption {
            value: ResolvedValue::User(user, Some(member)),
            ..
        }) = options.first()
        else {
            return funcs::error_msg(cirm, "Specify user");
        };
        let Some(joined_at) = member.joined_at else {
            return funcs::error_msg(cirm, "Unknown join date");
        };
        let repository = UserRepository::new(self.db_conn.clone());
        let user_data = data::UserData::new(user.id.into(), joined_at.into());

        if let Err(error) = repository.replace(user_data).await {
            log::error!("error: {error:?}");
            funcs::error_msg(cirm, "SQL error")
        } else {
            funcs::completed_msg(cirm, format!("Insert or replace user: {}", user.name))
        }
    }

    async fn list(
        &self,
        _ctx: &Context,
        _options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage {
        let repository = UserRepository::new(self.db_conn.clone());
        let users = repository.get_all().await;

        let Ok(users) = users else {
            log::error!("error: {:?}", users);
            return funcs::error_msg(cirm, "SQL Error");
        };
        cirm.content(format!("```\n{}\n```", Table::new(&users)))
    }
}

#[async_trait]
impl Command for NewUserCmd {
    fn register(&self, name: &str) -> CreateCommand {
        CreateCommand::new(name)
            .description("A new user command")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "replace",
                    "Insert or replace user",
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "users",
                )),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "list",
                "List all users",
            ))
    }
    async fn run(
        &self,
        ctx: &Context,
        options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage {
        let cirm = cirm.ephemeral(true);
        match options.first() {
            Some(ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "replace",
                ..
            }) => self.replace(ctx, options, cirm).await,
            Some(ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "list",
                ..
            }) => self.list(ctx, options, cirm).await,
            _ => funcs::error_msg(cirm, "Specify subcommand"),
        }
    }
}
