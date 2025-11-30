use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::domain::user_prems::{MemberUserPermissions, UserPermissions};
use crate::domain::validation;

use crate::auth;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NewUser {
    #[validate(
        length(min = 4, max = 16),
        custom(function = "validation::validate_alphanum")
    )]
    username: String,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: String,
    #[validate(length(min = 1, max = 64))]
    first_name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    last_name: Option<String>,
    permissions: Option<MemberUserPermissions>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InternalNewUser {
    pub uuid: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub permissions: Option<MemberUserPermissions>,
}

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct InternalUser {
    pub uuid: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[sqlx(skip)]
    pub permissions: Option<MemberUserPermissions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    uuid: Uuid,
    username: String,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug)]
pub enum UserConversionError {
    HashFailed(password_hash::Error),
}

impl TryFrom<NewUser> for InternalNewUser {
    type Error = UserConversionError;
    fn try_from(value: NewUser) -> Result<Self, Self::Error> {
        let password_hash =
            auth::hash_password(&value.password).map_err(|e| UserConversionError::HashFailed(e))?;
        let uuid = Uuid::new_v4();
        Ok(Self {
            uuid: uuid,
            username: value.username,
            email: value.email,
            password_hash: password_hash,
            first_name: value.first_name,
            last_name: value.last_name,
            permissions: value.permissions.clone(),
        })
    }
}

impl From<InternalUser> for User {
    fn from(value: InternalUser) -> Self {
        Self {
            uuid: value.uuid,
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
        }
    }
}

impl Display for UserConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserConversionError::HashFailed(e) => write!(f, "failed to hash password: {e}"),
        }
    }
}

impl InternalUser {
    pub fn attach_permissions(&mut self, permissions: UserPermissions) {
        self.permissions = Some(MemberUserPermissions::from(permissions));
    }
}
