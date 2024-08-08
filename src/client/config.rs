use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientConfig {
    pub version: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub client: ClientConfig,
}

impl Config {
    pub fn get_config() -> Config {
        Config {
            client: ClientConfig {
                version: "v1".to_string(),
            },
        }
    }
}
