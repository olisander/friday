use std::{net::SocketAddr, str::FromStr};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    host: String,
    port: u16,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Failed to read config")
    }
}

impl Config {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host, self.port))
            .expect("Failed to parse address")
    }
}
