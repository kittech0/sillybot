use serenity::all::{Context, CreateCommand, CreateInteractionResponseMessage, ResolvedOption};

use super::{newuser, ping, Command};
type C = Command;

impl C {
    pub async fn run(
        &self,
        ctx: &Context,
        options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage {
        match self {
            C::Ping => ping::run(ctx, options, cirm).await,
            C::NewUser => newuser::run(ctx, options, cirm).await,
        }
    }

    pub fn register(&self) -> CreateCommand {
        match self {
            C::Ping => ping::register(self.into()),
            C::NewUser => newuser::register(self.into()),
        }
    }
}
