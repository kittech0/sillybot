#![feature(type_alias_impl_trait)]
mod bot;
mod database;
mod util;
use bot::{
    commands::{CommandRegistry, MessagesCmd, NewUserCmd, PermissionCmd, PingCmd},
    BotHandler, CommandHandler, EventManager,
};
use database::{
    repository::{MessageRepository, PermissionRepository, UserRepository},
    DatabaseConnection,
};
use util::logger;

const GUILD_ID: u64 = 1285696315640123553;

#[tokio::main]
async fn main() -> util::ErrorResult {
    logger::init()?;

    let db_conn = DatabaseConnection::new(None)?;
    UserRepository::init(db_conn.clone()).await?;
    MessageRepository::init(db_conn.clone()).await?;
    PermissionRepository::init(db_conn.clone()).await?;
    let cmd_handler = CommandHandler::new(command_registry(db_conn.clone()));
    BotHandler::new(EventManager::new(db_conn.clone(), cmd_handler))
        .run()
        .await?;
    Ok(())
}

fn command_registry(db_conn: DatabaseConnection) -> CommandRegistry {
    let mut command_registry = CommandRegistry::new();
    command_registry.register("ping", PingCmd::new());
    command_registry.register("newuser", NewUserCmd::new(db_conn.clone()));
    command_registry.register("messages", MessagesCmd::new(db_conn.clone()));
    command_registry.register("permission", PermissionCmd::new(db_conn.clone()));
    command_registry
}
