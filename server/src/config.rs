use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,

    pub jwt_secret: String,

    pub max_connections: u32,

    pub port: u16,
}
