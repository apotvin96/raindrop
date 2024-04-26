#[derive(serde_derive::Deserialize, Clone)]
pub struct Config {
    pub info: InfoConfig,
    pub renderer: RendererConfig,
}

#[derive(serde_derive::Deserialize, Clone)]
pub struct InfoConfig {
    pub name: String,
}

#[derive(serde_derive::Deserialize, Clone)]
pub struct RendererConfig {
    pub vsync: bool,
    pub window_width: u32,
    pub window_height: u32,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        let contents = std::fs::read_to_string(path).expect("Failed to load config file");

        let config_data: Config = toml::from_str(&contents).expect("Failed to parse config file");

        config_data
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            info: InfoConfig {
                name: "Raindrop Engine".to_string(),
            },
            renderer: RendererConfig {
                vsync: true,
                window_width: 800,
                window_height: 600,
            },
        }
    }
}
