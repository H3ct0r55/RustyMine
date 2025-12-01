use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
pub enum UserActions {
    Root,
    ManageUsers,
    Login,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct UserPermissions {
    pub root: bool,
    pub manage_users: bool,
    pub login: bool,
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self {
            root: false,
            manage_users: false,
            login: false,
        }
    }
}

impl UserPermissions {
    pub fn root() -> Self {
        Self {
            root: true,
            manage_users: true,
            login: true,
        }
    }
}
