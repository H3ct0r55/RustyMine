#[derive(Debug, Clone)]
pub struct AppCfg {
    pub master_path: String,
    pub db_path: String,
    pub servers_path: String,
}

impl AppCfg {
    pub fn new() -> Self {
        Self {
            master_path: "/home/hector/rmtest/var/opt/rustymine/".to_string(),
            db_path: "main.db3".to_string(),
            servers_path: "servers".to_string(),
        }
    }
}
