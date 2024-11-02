use std::fmt::Display;

use chrono::format::Item;

use super::{ControlAccess, Identifier, ItemData, ItemType, SqlData};

impl ItemData {
    fn new(name: impl Into<String>, item_type: ItemType, control_access: ControlAccess) -> Self {
        Self {
            name: Identifier(name.into()),
            item_type,
            control_access,
        }
    }
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ItemType::Command => "command",
            }
        )
    }
}

impl SqlData for Identifier {
    fn get_sql_type() -> impl AsRef<str> {
        "TEXT NOT NULL"
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
