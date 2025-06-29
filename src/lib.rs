use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    CONFIG
        .set(config)
        .map_err(|_| "Config already initialized".into())
}

pub fn get() -> &'static Config {
    CONFIG
        .get()
        .expect("Config is not initialized. Call init() first.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_deserialize() {
        let toml_str = r#"
                    [database]
                    host = "localhost"
                    port = 5432
                "#;
        let config: Config = toml::from_str(toml_str).expect("Failed to parse");
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
    }
}
