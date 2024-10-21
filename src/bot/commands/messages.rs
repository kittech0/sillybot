use serenity::{
    all::{
        CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
    },
    async_trait,
};
use tabled::Table;

use crate::{
    database::{repository::MessagesRepository, DatabaseConnection},
    util::funcs,
};

use super::{Command, MessagesCmd};

impl MessagesCmd {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }

    async fn list(
        &self,
        _ctx: &Context,
        _options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage {
        let repository = MessagesRepository::new(self.db_conn.clone());
        let messages = repository.get_all().await;

        let Ok(messages) = messages else {
            log::error!("error: {:?}", messages.err());
            return funcs::error_msg(cirm, "SQL Error");
        };
        cirm.content(format!("```\n{}\n```", Table::new(&messages)))
    }
}

#[async_trait]
impl Command for MessagesCmd {
    fn register(&self, name: &str) -> CreateCommand {
        CreateCommand::new(name)
            .description("A new user command")
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "list",
                "List all messages",
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
                name: "list",
                ..
            }) => self.list(ctx, options, cirm).await,
            _ => funcs::error_msg(cirm, "Specify subcommand"),
        }
    }
}
