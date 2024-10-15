use core::error;
use std::str::FromStr;

use chrono::NaiveDateTime;
use serenity::all::{
    Colour, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponseMessage, PartialMember, ResolvedOption, ResolvedValue, User, UserId,
};
use tabled::Table;

use crate::bot::database::{self, data, DatabaseHandler, UserRepository};

pub fn register(name: &str) -> CreateCommand {
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
pub async fn run(
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
        }) => replace(ctx, options, cirm).await,
        Some(ResolvedOption {
            value: ResolvedValue::SubCommand(options),
            name: "list",
            ..
        }) => list(ctx, options, cirm).await,
        _ => error_msg(cirm, "Specify subcommand"),
    }
}

pub async fn replace(
    _ctx: &Context,
    options: &[ResolvedOption<'_>],
    cirm: CreateInteractionResponseMessage,
) -> CreateInteractionResponseMessage {
    let Some(ResolvedOption {
        value: ResolvedValue::User(user, Some(member)),
        ..
    }) = options.first()
    else {
        return error_msg(cirm, "Specify user");
    };
    let Some(joined_at) = member.joined_at else {
        return error_msg(cirm, "Unknown join date");
    };

    let join_date = NaiveDateTime::new(joined_at.date_naive(), joined_at.time());
    let repository = UserRepository::new().await;
    let user_data = data::User::new(user.id.get(), join_date);

    if let Err(error) = repository.replace(user_data).await {
        log::error!("Error: {error:?}");
        error_msg(cirm, "SQL Error")
    } else {
        completed_msg(cirm, format!("Insert or replace user: {}", user.name))
    }
}

pub async fn list(
    ctx: &Context,
    _options: &[ResolvedOption<'_>],
    cirm: CreateInteractionResponseMessage,
) -> CreateInteractionResponseMessage {
    let repository = UserRepository::new().await;
    let users = repository.get_all().await;

    let Ok(users) = users else {
        log::error!("Error: {:?}", users.err().unwrap());
        return error_msg(cirm, "SQL Error");
    };
    cirm.content(format!("```\n{}\n```", Table::new(&users)))
}

fn error_msg(
    cirm: CreateInteractionResponseMessage,
    msg: impl Into<String>,
) -> CreateInteractionResponseMessage {
    cirm.embed(
        CreateEmbed::new()
            .color(Colour::ROSEWATER)
            .title("Error")
            .description(msg),
    )
}

fn completed_msg(
    cirm: CreateInteractionResponseMessage,
    msg: impl Into<String>,
) -> CreateInteractionResponseMessage {
    cirm.embed(
        CreateEmbed::new()
            .color(Colour::BLURPLE)
            .title("Completed")
            .description(msg),
    )
}
