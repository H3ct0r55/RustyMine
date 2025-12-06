use axum::{
    Json, RequestExt,
    extract::{MatchedPath, Request, State},
    http::{Method, StatusCode},
};
use std::collections::HashMap;

use crate::domain::{
    user::NewUser,
    user_prems::{InternalUserPermissions, UserActions, UserPermissions},
};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct RouteKey {
    pub method: Method,
    pub path: String,
}

#[derive(Debug)]
pub struct AppCfg {
    pub db_path: String,
    pub route_perms: HashMap<RouteKey, InternalUserPermissions>,
}

impl AppCfg {
    pub fn new(db_path: String) -> Self {
        Self {
            db_path,
            route_perms: HashMap::new(),
        }
    }

    pub fn insert_route_perms(
        &mut self,
        method: Method,
        path: impl Into<String>,
        root: bool,
        perms: Vec<UserActions>,
        esc_check: bool,
    ) {
        let key = RouteKey {
            method,
            path: path.into(),
        };

        let user_perms = InternalUserPermissions {
            root,
            permissions: perms.into_iter().collect(), // Vec → HashSet
            esc_check,
        };

        self.route_perms.insert(key, user_perms);
    }

    pub fn get_route_perms(&self, method: &Method, path: &str) -> Option<InternalUserPermissions> {
        let key = RouteKey {
            method: method.clone(),
            path: path.to_string(),
        };

        let perm = match self.route_perms.get(&key) {
            Some(val) => val.clone(),
            None => return None,
        };

        Some(perm)
    }

    pub async fn route_allows(
        &self,
        req: Request,
        user_perms: UserPermissions,
    ) -> Result<bool, StatusCode> {
        let method = req.method();

        let path = req
            .extensions()
            .get::<MatchedPath>()
            .map(|p| p.as_str())
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let req_perms = match self.get_route_perms(method, path) {
            Some(val) => val,
            None => return Ok(false),
        };

        if req_perms.root {
            return Ok(true);
        }

        match req_perms
            .permissions
            .iter()
            .all(|action| user_perms.permissions.contains(action))
        {
            true => (),
            false => return Ok(false),
        };

        if req_perms.esc_check {
        } else {
            Ok(true)
        }
    }
}
