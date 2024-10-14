use serenity::all::{
    Context, CreateCommand, CreateEmbed, CreateInteractionResponseMessage, ResolvedOption,
};

pub fn register(name: &str) -> CreateCommand {
    CreateCommand::new(name).description("A ping command")
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
