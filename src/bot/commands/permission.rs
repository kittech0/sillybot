use serenity::{
    all::{
        CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
    },
    async_trait,
};
use tabled::Table;

use crate::{
    database::{
        data::{self, ControlAccess},
        repository::PermissionRepository,
        DatabaseConnection,
    },
    util::funcs,
};

use super::{Command, PermissionCmd};
impl PermissionCmd {
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
            value: ResolvedValue::String(name),
            ..
        }) = options.first()
        else {
            return funcs::error_msg(cirm, "Specify permission name");
        };
        let Some(ResolvedOption {
            value: ResolvedValue::String(access),
            ..
        }) = options.get(1)
        else {
            return funcs::error_msg(cirm, "Specify access");
        };
        let access_a = match *access {
            "deny" => ControlAccess::Disallow,
            "allow" => ControlAccess::Allow,
            _ => return funcs::error_msg(cirm, "Specify access"),
        };
        let repository = PermissionRepository::get(self.db_conn.clone());
        let user_data = data::PermissionData::new(*name, access_a);

        if let Err(error) = repository.replace(user_data).await {
            log::error!("error: {error:?}");
            funcs::error_msg(cirm, "SQL error")
        } else {
            funcs::completed_msg(cirm, format!("Insert or replace permission: {}", *name))
        }
    }

    async fn list(
        &self,
        _ctx: &Context,
        _options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage {
        let repository = PermissionRepository::get(self.db_conn.clone());
        let users = repository.get_all().await;

        let Ok(users) = users else {
            log::error!("error: {:?}", users);
            return funcs::error_msg(cirm, "SQL Error");
        };
        cirm.content(format!("```\n{}\n```", Table::new(&users)))
    }
}

#[async_trait]
impl Command for PermissionCmd {
    fn register(&self, name: &str) -> CreateCommand {
        CreateCommand::new(name)
            .description("A new permission command")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "replace",
                    "Insert or replace permission",
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::String,
                    "name",
                    "name",
                ))
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "default", "default value")
                        .add_string_choice("allow", "allow")
                        .add_string_choice("deny", "deny"),
                ),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "list",
                "List all permissions",
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
