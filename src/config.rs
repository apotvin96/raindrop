#[derive(serde_derive::Deserialize)]
pub struct Config {
    pub info: InfoConfig,
    pub renderer: RendererConfig,
}

#[derive(serde_derive::Deserialize)]
pub struct InfoConfig {
    pub title: String
}

#[derive(serde_derive::Deserialize)]
pub struct RendererConfig {
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
