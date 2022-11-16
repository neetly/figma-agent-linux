use std::env;

pub struct Config {
    pub enable_variable_font: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            enable_variable_font: env::var("FIGMA_AGENT_VARIABLE_FONT")
                .map(|value| value != "false" && value != "0")
                .unwrap_or(true),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
