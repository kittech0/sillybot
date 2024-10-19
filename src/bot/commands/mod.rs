use std::collections::HashMap;

use serenity::{
    all::{Context, CreateCommand, CreateInteractionResponseMessage, ResolvedOption},
    async_trait,
};

use crate::database::DatabaseConnection;

pub mod handler;
pub mod newuser;
pub mod ping;
pub mod registry;

pub struct CommandRegistry(HashMap<String, Box<dyn Command>>);

#[async_trait]
pub trait Command: Send + Sync {
    fn register(&self, name: &str) -> CreateCommand;

    async fn run(
        &self,
        ctx: &Context,
        options: &[ResolvedOption<'_>],
        cirm: CreateInteractionResponseMessage,
    ) -> CreateInteractionResponseMessage;
}

pub struct NewUserCmd {
    db_conn: DatabaseConnection,
}

pub struct PingCmd {}
