use super::DatabaseConnection;

pub mod messages;
pub mod user;
pub struct UserRepository {
    db_conn: DatabaseConnection,
}

pub struct MessagesRepository {
    db_conn: DatabaseConnection,
}
