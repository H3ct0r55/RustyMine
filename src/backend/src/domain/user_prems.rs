use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum UserActions {
    ManageUsers,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Default)]
pub struct UserPermissions {
    pub root: bool,
    pub permissions: HashSet<UserActions>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserPermissionsRow {
    pub root: bool,
    pub permissions: Json<HashSet<UserActions>>,
}

impl From<UserPermissions> for UserPermissionsRow {
    fn from(value: UserPermissions) -> Self {
        Self {
            root: value.root,
            permissions: Json(value.permissions),
        }
    }
}

impl From<UserPermissionsRow> for UserPermissions {
    fn from(value: UserPermissionsRow) -> Self {
        Self {
            root: value.root,
            permissions: value.permissions.0,
        }
    }
}


impl UserPermissions {
    pub fn root() -> Self {
        Self {
            root: true,
            permissions: HashSet::new(),
        }
    }
}
