use std::collections::HashMap;

use serenity::all::{Context, CreateCommand, CreateInteractionResponseMessage, ResolvedOption};

use super::{newuser, ping, Command, CommandRegistry};

impl CommandRegistry {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add(&mut self, name: impl Into<String>, command: Box<dyn Command>) {
        self.0.insert(name.into(), command);
    }
    pub async fn run(
        &self,
        name: impl AsRef<str>,
        ctx: &Context,
        options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> Option<CreateInteractionResponseMessage> {
        let cmd = self.0.get(name.as_ref())?;
        Some(cmd.run(ctx, options, cirm).await)
    }
}
