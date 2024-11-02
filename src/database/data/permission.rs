use std::fmt::Display;

use rusqlite::Row;

use super::{ControlAccess, Identifier, PermissionData, SqlData};

impl PermissionData {
    pub fn new(name: impl Into<String>, default_value: ControlAccess) -> Self {
        Self {
            name: Identifier(name.into()),
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
            name: Identifier(name),
            default_value: default_value.into(),
        })
    }
}

impl From<bool> for ControlAccess {
    fn from(value: bool) -> Self {
        match value {
            true => ControlAccess::Allow,
            false => ControlAccess::Disallow,
        }
    }
}

impl Display for ControlAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ControlAccess::Allow => "allow",
                ControlAccess::Disallow => "disallow",
            },
        )
    }
}

impl SqlData for ControlAccess {
    fn get_sql_type() -> impl AsRef<str> {
        "INT NOT NULL"
    }
}
