use serenity::{
    all::{Context, CreateCommand, CreateInteractionResponseMessage, ResolvedOption},
    async_trait,
};
use strum::{EnumIter, EnumString, IntoStaticStr};
pub mod cmd;
pub mod newuser;
pub mod ping;

#[async_trait]
pub trait CommandRunner: Sync + Send {
    async fn run(&self, ctx: &Context, options: &[ResolvedOption]) -> String;
}

pub trait CommandRegister {
    fn register(name: &str) -> CreateCommand;
    fn options(cirm: CreateInteractionResponseMessage) -> CreateInteractionResponseMessage;
}

#[derive(EnumIter, EnumString, IntoStaticStr)]
pub enum Command {
    #[strum(serialize = "ping")]
    Ping(PingCommand),
    #[strum(serialize = "newuser")]
    NewUser(NewUserCommand),
}

#[derive(Default)]
pub struct PingCommand;
#[derive(Default)]
pub struct NewUserCommand;
