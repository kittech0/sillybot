use super::DatabaseConnection;

pub mod user;
pub struct UserRepository {
    db_conn: DatabaseConnection,
}

