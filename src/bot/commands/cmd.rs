use serenity::all::{CreateCommand, CreateInteractionResponseMessage};

use super::{Command, CommandRegister, CommandRunner, NewUserCommand, PingCommand};
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
            C::Ping(_) => PingCommand::register(self.into()),
            C::NewUser(_) => NewUserCommand::register(self.into()),
        }
    }
    pub fn options(&self) -> CreateInteractionResponseMessage {
        match self {
            C::Ping(_) => PingCommand::options(),
            C::NewUser(_) => NewUserCommand::options(),
        }
    }
}
