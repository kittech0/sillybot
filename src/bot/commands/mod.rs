use strum::{EnumIter, EnumString, IntoStaticStr};
pub mod cmd;
pub mod handler;
pub mod newuser;
pub mod ping;

#[derive(EnumIter, EnumString, IntoStaticStr)]
pub enum CommandStore {
    #[strum(serialize = "ping")]
    Ping,
    #[strum(serialize = "newuser")]
    NewUser,
}
