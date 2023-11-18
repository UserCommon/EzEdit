use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    ip: String,
    port: String,
}

impl Config {
    pub fn from_config() -> Self {
        let config = include_str!("config.json");
        serde_json::from_str(config).expect("Failed to open config.json!")
    }

    pub fn get_url(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}