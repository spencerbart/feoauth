use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub jwt_secret: String,

    pub port: u16,
}
