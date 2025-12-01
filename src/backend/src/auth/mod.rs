use crate::prelude::*;
use std::i64;

use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

use crate::domain::api::AuthClaims;

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    debug!("password hashed");
    Ok(hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => return Ok(true),
        Err(password_hash::Error::Password) => return Ok(false),
        Err(e) => Err(e),
    }
}

pub fn gen_jwt(username: String) -> Result<String, StatusCode> {
    // TODO: Update secret to .env file
    let secret: String = "verysafestring".to_string();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp = (now + expire).timestamp() as i64;
    let iat = now.timestamp() as i64;
    let claim = AuthClaims { iat, exp, username };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| {
        error!(error = %e, username = claim.username, "create jwt failed");
        return StatusCode::INTERNAL_SERVER_ERROR;
    })
}

pub fn verify_jwt(token: String) -> Result<TokenData<AuthClaims>, StatusCode> {
    let secret = "verysafestring".to_string();
    let result: Result<TokenData<AuthClaims>, StatusCode> = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        error!(error = %e, "verify jwt failed");
        return StatusCode::INTERNAL_SERVER_ERROR;
    });
    result
}
