use axum::{
    Json, RequestExt,
    extract::{MatchedPath, Request, State},
    http::{Method, StatusCode},
};
use std::collections::HashMap;

use crate::domain::user_prems::{UserActions, UserPermissions};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct RouteKey {
    pub method: Method,
    pub path: String,
}

#[derive(Debug)]
pub struct AppCfg {
    pub db_path: String,
    pub route_perms: HashMap<RouteKey, UserPermissions>,
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
    ) {
        let key = RouteKey {
            method,
            path: path.into(),
        };

        let user_perms = UserPermissions {
            root,
            permissions: perms.into_iter().collect(), // Vec → HashSet
        };

        self.route_perms.insert(key, user_perms);
    }

    pub fn get_route_perms(&self, method: &Method, path: &str) -> Option<UserPermissions> {
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
        method: &Method,
        path: &str,
        user_perms: UserPermissions,
    ) -> Result<bool, StatusCode> {
        let req_perms = match self.get_route_perms(method, path) {
            Some(val) => val,
            None => return Ok(false),
        };

        if req_perms.root {
            return Ok(true);
        }

        Ok(req_perms
            .permissions
            .iter()
            .all(|action| user_perms.permissions.contains(action)))
    }
}
