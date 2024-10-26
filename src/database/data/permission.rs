use std::fmt::{write, Display};

use rusqlite::Row;

use super::{PermissionData, PermissionName, PermissionValue, SqlData};

impl PermissionData {
    pub fn new(name: String, default_value: PermissionValue) -> Self {
        Self {
            name,
            default_value,
        }
    }
}

impl TryFrom<&Row<'_>> for PermissionData {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        let name: String = row.get(0)?;
        let default_value: bool = row.get(1)?;

        Ok(Self {
            name,
            default_value: default_value.into(),
        })
    }
}

impl From<bool> for PermissionValue {
    fn from(value: bool) -> Self {
        match value {
            true => PermissionValue::Allow,
            false => PermissionValue::Disallow,
        }
    }
}

impl Display for PermissionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PermissionValue::Allow => 1,
                PermissionValue::Disallow => 0,
            },
        )
    }
}

impl SqlData for PermissionName {
    fn get_sql_type() -> impl AsRef<str> {
        "TEXT NOT NULL"
    }
}

impl SqlData for PermissionValue {
    fn get_sql_type() -> impl AsRef<str> {
        "INT NOT NULL"
    }
}
