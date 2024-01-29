pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        // TODO: Actually load from a config file
        Config {
            window_width: 800,
            window_height: 600,
        }
    }
}
