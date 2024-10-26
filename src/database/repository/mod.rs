use super::DatabaseConnection;

pub mod message;
pub mod permission;
pub mod user;
pub struct UserRepository {
    db_conn: DatabaseConnection,
}

pub struct MessageRepository {
    db_conn: DatabaseConnection,
}

pub struct PermissionRepository {
    db_conn: DatabaseConnection,
}
