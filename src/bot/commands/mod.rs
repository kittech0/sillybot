use serenity::{
    all::{
        CommandOptionType, CreateCommand, CreateCommandOption, CreateInteractionResponseMessage,
        ResolvedOption,
    },
    async_trait,
};
use strum::{EnumIter, EnumString, IntoStaticStr};
pub mod ping;
pub mod testinput;

#[async_trait]
pub trait CommandRunner: Sync + Send {
    async fn run(&self, options: &[ResolvedOption]) -> String;
}
#[derive(EnumIter, EnumString, IntoStaticStr)]
pub enum Command {
    #[strum(serialize = "ping")]
    Ping(PingCommand),
    #[strum(serialize = "testinput")]
    TestInput(TestInputCommand),
}
type C = Command;

impl Command {
    pub fn runner(&self) -> &dyn CommandRunner {
        match self {
            C::Ping(c, ..) => c,
            C::TestInput(c, ..) => c,
        }
    }
    pub fn register(&self) -> CreateCommand {
        match self {
            C::Ping(_) => PingCommand::register(),
            C::TestInput(_) => TestInputCommand::register(),
        }
    }
    pub fn options(&self) -> CreateInteractionResponseMessage {
        match self {
            C::Ping(_) => PingCommand::options(),
            C::TestInput(_) => TestInputCommand::options(),
        }
    }
}
#[derive(Default)]
pub struct PingCommand;
#[derive(Default)]
pub struct TestInputCommand;
