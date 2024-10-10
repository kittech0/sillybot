use serenity::{
    all::{CreateCommand, CreateInteractionResponseMessage, ResolvedOption},
    async_trait,
};
pub mod ping;
pub mod testinput;

#[async_trait]
pub trait Command: Sync + Send {
    async fn run(&self, options: &[ResolvedOption]) -> String;
    async fn register(&self) -> CreateCommand;
    async fn options(&self) -> CreateInteractionResponseMessage;
}
