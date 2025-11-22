#[derive(Debug, Clone)]
pub struct AppCfg {
    pub db_path: String,
}

impl AppCfg {
    pub fn new() -> Self {
        Self {
            db_path: "/home/hector/rmtest/var/opt/rustymine/main.db3".to_string(),
        }
    }
}
