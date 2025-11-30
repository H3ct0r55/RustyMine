use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug)]
pub enum UserActions {
    Root,
    ManageUsers,
    Login,
}

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct UserPermissions {
    pub uuid: Uuid,
    pub root: bool,
    pub manage_users: bool,
    pub login: bool,
}

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct MemberUserPermissions {
    pub root: bool,
    pub manage_users: bool,
    pub login: bool,
}

impl From<UserPermissions> for MemberUserPermissions {
    fn from(value: UserPermissions) -> Self {
        Self {
            root: value.root,
            manage_users: value.manage_users,
            login: value.login,
        }
    }
}
