use config::Config;
use config::File;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub port: u16,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    pub allowed_kinds: Vec<u64>,
    pub allowed_authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub network: Network,
    pub rules: Rules,
}

impl Settings {
    pub fn new() -> Self {
        let builder = Config::builder();
        let mut settings = Settings::default();
        let config = builder
            .add_source(Config::try_from(&settings).unwrap())
            .add_source(File::with_name("config.toml"))
            .build()
            .unwrap();
        
        settings = config.try_deserialize().unwrap();
        settings
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            network: Network {
                port: 50051,
                address: "[::1]".to_string(),
            },
            rules: Rules {
                allowed_kinds: vec![0, 1, 2, 3, 30023],
                allowed_authors: vec![],
            },
        }
    }
}