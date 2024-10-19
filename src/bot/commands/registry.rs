use std::collections::HashMap;

use serenity::all::{Context, CreateInteractionResponseMessage, ResolvedOption};

use super::{Command, CommandRegistry};

impl CommandRegistry {
    pub fn new<const T: usize>(values: [(String, Box<dyn Command>); T]) -> Self {
        Self(HashMap::from(values))
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
