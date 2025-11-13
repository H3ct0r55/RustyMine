use ::anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppCfg {
    pub data_path: String,
    pub sock_path: String,
    pub db_path: String,
}

impl AppCfg {
    pub fn load() -> Result<Self> {
        Ok(Self {
            data_path: "/home/hector/rmtest/var/opt/rustymine".to_string(),
            sock_path: "/home/hector/rmtest/run/rustymine.sock".to_string(),
            db_path: "/home/hector/rmtest/var/opt/rustymine/main.db3".to_string(),
        })
    }
}
