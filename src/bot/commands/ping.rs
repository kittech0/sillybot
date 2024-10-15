use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponseMessage, ResolvedOption,
};

pub fn register(name: &str) -> CreateCommand {
    CreateCommand::new(name)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommandGroup, "test", "test")
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "lol",
                    "silly",
                )),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommandGroup, "test2", "test")
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "lol2",
                    "silly",
                )),
        )
        .description("A ping command")
}

pub async fn run(
    _ctx: &Context,
    _options: &[ResolvedOption<'_>],
    cirm: CreateInteractionResponseMessage,
) -> CreateInteractionResponseMessage {
    let content = format!("PING {:?}", 0);
    let embed = CreateEmbed::new().title("Test");
    cirm.ephemeral(true).add_embed(embed).content(content)
}
