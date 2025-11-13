use ::anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppCfg {
    pub data_path: String,
    pub sock_path: String,
}

impl AppCfg {
    pub fn load() -> Result<Self> {
        Ok(Self {
            data_path: "/var/opt/rustymine".to_string(),
            sock_path: "/run/rustymine.sock".to_string(),
        })
    }
}
