use std::{fmt::format, sync::Arc};

use serenity::{
    all::{Context, CreateCommand, CreateInteractionResponseMessage, ResolvedOption, ShardManager},
    async_trait, client,
    prelude::TypeMapKey,
};

use super::{CommandRegister, CommandRunner, PingCommand};

impl CommandRegister for PingCommand {
    fn register(name: &str) -> CreateCommand {
        CreateCommand::new(name).description("A ping command")
    }

    fn options(cirm: CreateInteractionResponseMessage) -> CreateInteractionResponseMessage {
        cirm.ephemeral(true)
    }
}

// pub struct ShardManagerContainer;

// impl TypeMapKey for ShardManagerContainer {
//     type Value = Arc<ShardManager>;
// }

#[async_trait]
impl CommandRunner for PingCommand {
    async fn run(&self, _ctx: &Context, _options: &[ResolvedOption]) -> String {
        format!("PING {:?}", 0)
    }
}
