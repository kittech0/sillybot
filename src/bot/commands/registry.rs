use std::collections::HashMap;

use serenity::all::{Context, CreateInteractionResponseMessage, ResolvedOption};

use super::{Command, CommandRegistry};

impl CommandRegistry {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register(&mut self, name: impl Into<String>, cmd: impl Command + 'static) {
        self.0.insert(name.into(), Box::new(cmd));
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
