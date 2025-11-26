use once_cell::sync::Lazy;
use serde::Deserialize;

const RAW_CONFIG: &str = include_str!("../../App.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct AppSection {
    pub name: String,
    pub terminal_window: bool,
    pub mac_title_bar: bool,
    pub prompt_placeholder: String,
    pub prompt_symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthorSection {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app: AppSection,
    pub author: AuthorSection,
}

pub struct ConfigService;

impl ConfigService {
    pub fn get() -> &'static AppConfig {
        &CONFIG
    }
}

pub static CONFIG: Lazy<AppConfig> =
    Lazy::new(|| toml::from_str(RAW_CONFIG).expect("Failed to parse App.toml into AppConfig"));
