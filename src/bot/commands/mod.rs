use serenity::{all::ResolvedOption, async_trait};
use strum::{EnumIter, EnumString, IntoStaticStr};
pub mod cmd;
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

#[derive(Default)]
pub struct PingCommand;
#[derive(Default)]
pub struct TestInputCommand;
