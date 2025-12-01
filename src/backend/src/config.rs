use axum::http::Method;
use std::collections::HashMap;

use crate::domain::user_prems::UserActions;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct RouteKey {
    pub method: Method,
    pub path: String,
}

#[derive(Debug)]
pub struct AppCfg {
    pub db_path: String,
    pub route_perms: HashMap<RouteKey, Vec<UserActions>>,
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
        perms: Vec<UserActions>,
    ) {
        let key = RouteKey {
            method,
            path: path.into(),
        };

        self.route_perms.insert(key, perms);
    }

    pub fn get_route_perms(&self, method: &Method, path: &str) -> Option<&Vec<UserActions>> {
        let key = RouteKey {
            method: method.clone(),
            path: path.to_string(),
        };

        self.route_perms.get(&key)
    }

    pub fn route_allows(&self, method: &Method, path: &str, user_perms: &[UserActions]) -> bool {
        if user_perms.contains(&UserActions::Root) {
            return true;
        }

        let key = RouteKey {
            method: method.clone(),
            path: path.to_string(),
        };

        let required = match self.route_perms.get(&key) {
            Some(perms) => perms,
            None => return true, // no perms required → allow
        };

        if required.contains(&UserActions::Root) {
            return false;
        }

        required.iter().all(|p| user_perms.contains(p))
    }
}
