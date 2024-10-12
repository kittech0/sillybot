use serenity::all::{CreateCommand, CreateInteractionResponseMessage};

use super::{Command, CommandRunner, NewUserCommand, PingCommand};
type C = Command;

impl Command {
    pub fn runner(&self) -> &dyn CommandRunner {
        match self {
            C::Ping(c, ..) => c,
            C::NewUser(c, ..) => c,
        }
    }
    pub fn register(&self) -> CreateCommand {
        match self {
            C::Ping(_) => PingCommand::register(),
            C::NewUser(_) => NewUserCommand::register(),
        }
    }
    pub fn options(&self) -> CreateInteractionResponseMessage {
        match self {
            C::Ping(_) => PingCommand::options(),
            C::NewUser(_) => NewUserCommand::options(),
        }
    }
}
