use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum UserActions {
    ManageUsers,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UserPermissions {
    pub root: bool,
    pub permissions: HashSet<UserActions>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct InternalUserPermissions {
    pub root: bool,
    pub permissions: HashSet<UserActions>,
    pub esc_check: bool,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ExtUserPermissions {
    pub uuid: Uuid,
    pub root: bool,
    pub permissions: HashSet<UserActions>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserPermissionsRow {
    pub root: bool,
    pub permissions: Json<HashSet<UserActions>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ExtUserPermissionsRow {
    pub uuid: Uuid,
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

impl From<ExtUserPermissionsRow> for ExtUserPermissions {
    fn from(value: ExtUserPermissionsRow) -> Self {
        Self {
            uuid: value.uuid,
            root: value.root,
            permissions: value.permissions.0,
        }
    }
}

impl From<ExtUserPermissions> for UserPermissions {
    fn from(value: ExtUserPermissions) -> Self {
        Self {
            root: value.root,
            permissions: value.permissions,
        }
    }
}

impl UserPermissions {
    pub fn new() -> Self {
        Self {
            root: false,
            permissions: HashSet::new(),
        }
    }
    pub fn root() -> Self {
        Self {
            root: true,
            permissions: HashSet::new(),
        }
    }
}
