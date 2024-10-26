use rusqlite::params;

use crate::{
    database::{
        data::{self, SqlData},
        DatabaseConnection,
    },
    util::{self, ErrorResult},
};

use super::PermissionRepository;

impl PermissionRepository {
    pub fn get(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }

    pub async fn init(db_conn: DatabaseConnection) -> ErrorResult {
        db_conn.lock().await.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS Permissions (
                    name {} UNIQUE,
                    default_value {}
                ) STRICT",
                data::PermissionName::get_sql_type().as_ref(),
                data::PermissionValue::get_sql_type().as_ref()
            ),
            (),
        )?;
        log::warn!("loading permissions repository");

        Ok(())
    }

    pub async fn replace(&self, permission_data: data::PermissionData) -> ErrorResult {
        let conn = self.db_conn.lock().await;
        conn.execute(
            "REPLACE INTO Permissions (name,default_value) VALUES (?1,?)",
            params![
                permission_data.name.to_string(),
                permission_data.default_value.to_string(),
            ],
        )?;
        Ok(())
    }

    pub async fn get_all(&self) -> ErrorResult<Vec<data::PermissionData>> {
        let conn = self.db_conn.lock().await;
        let mut stmt = conn.prepare("SELECT name,default_value FROM Permissions")?;
        let user_iter = stmt.query_map([], |r| data::PermissionData::try_from(r))?;
        user_iter.map(|r| r.map_err(util::Error::from)).collect()
    }
}
