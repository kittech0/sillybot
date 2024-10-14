use serenity::all::{Context, CreateCommand, CreateInteractionResponseMessage, ResolvedOption};

pub fn register(name: &str) -> CreateCommand {
    CreateCommand::new(name).description("A ping command")
}

pub fn options(cirm: CreateInteractionResponseMessage) -> CreateInteractionResponseMessage {
    cirm.ephemeral(true)
}
pub async fn run(_ctx: &Context, _options: &[ResolvedOption<'_>]) -> String {
    format!("PING {:?}", 0)
}
