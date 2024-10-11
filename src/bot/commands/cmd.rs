use serenity::all::{CreateCommand, CreateInteractionResponseMessage};

use super::{Command, CommandRunner, PingCommand, TestInputCommand};
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
