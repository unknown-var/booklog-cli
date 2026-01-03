use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;
use toml;

static CONFIG: OnceLock<General> = OnceLock::new();

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default = "default_general")]
    general: General,
}

// Default function
fn default_general() -> General {
    General {
        data_path: default_data_path(),
        data_dir_name: default_dir_name(),
        book_file: default_book_file(),
        text_editor: default_text_editor(),
    }
}
#[derive(Debug, Deserialize)]
pub struct General {
    #[serde(default = "default_data_path")]
    pub data_path: String,
    #[serde(default = "default_dir_name")]
    pub data_dir_name: String,
    #[serde(default = "default_book_file")]
    pub book_file: String,
    #[serde(default = "default_text_editor")]
    pub text_editor: String,
}

// Default functions
fn default_data_path() -> String {
    "~".to_string()
}
fn default_dir_name() -> String {
    ".booklog-cli-data".to_string()
}
fn default_book_file() -> String {
    "books.csv".to_string()
}

fn default_text_editor() -> String {
    "vim".to_string()
}

fn load_settings(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn init_config() {
    let expanded_path = shellexpand::tilde("~/.config/booklog-cli/config.toml");
    let path = expanded_path.to_string();
    let loaded_config = match load_settings(&path) {
        Ok(config) => config.general,
        Err(_) => {
            let config: Config = toml::from_str("").expect("could not parse default config");
            config.general
        }
    };

    CONFIG
        .set(loaded_config)
        .expect("Error: trying to init config twice");
}

pub fn get_config() -> &'static General {
    CONFIG
        .get()
        .expect("Error: trying to acces non initialized config!")
}
