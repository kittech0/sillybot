use std::{future::Future, pin::Pin};

use serenity::all::{Context, CreateCommand, CreateInteractionResponseMessage, ResolvedOption};

use super::{newuser, ping, Command};
type C = Command;

impl C {
    pub async fn run(&self, ctx: &Context, options: &[ResolvedOption<'_>]) -> String {
        match self {
            C::Ping => ping::run(ctx, options).await,
            C::NewUser => newuser::run(ctx, options).await,
        }
    }

    pub fn register(&self) -> CreateCommand {
        match self {
            C::Ping => ping::register(self.into()),
            C::NewUser => newuser::register(self.into()),
        }
    }
    pub fn options(
        &self,
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage {
        match self {
            C::Ping => ping::options(cirm),
            C::NewUser => newuser::options(cirm),
        }
    }
}
