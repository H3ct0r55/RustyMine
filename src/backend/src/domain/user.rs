use std::fmt::Display;

use axum::response::IntoResponse;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
}

#[derive(Debug, Clone, Deserialize)]
pub struct InternalNewUser {
    uuid: Uuid,
    username: String,
    email: Option<String>,
    password_hash: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InternalUser {
    uuid: Uuid,
    username: String,
    email: Option<String>,
    password_hash: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
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
