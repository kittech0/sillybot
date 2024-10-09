mod pings {
    use serenity::all::{CreateCommand, ResolvedOption};

    pub fn run(_options: &[ResolvedOption]) -> String {
        "Hey, I'm alive!".to_string()
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("ping").description("A ping command")
    }
}
