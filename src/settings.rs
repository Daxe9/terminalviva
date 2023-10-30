use crate::api::TokenCredential;
use config::{Config, File};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env::{consts, var};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSettings {
    #[serde(alias = "wrap-width")]
    pub wrap_width: usize,
}

pub struct UserConfig {
    pub raw_body: Config,
    pub default_headers: HeaderMap,
    pub user_settings: ConfigSettings,
    // .credntials.json file and config.toml file
    pub paths: (PathBuf, PathBuf),
}

#[derive(Serialize, Deserialize, Debug)]
struct DefaultHeaders {
    pub key: String,
    pub value: String,
}

fn allowed_linux() {
    match consts::OS {
        "linux" => (),
        _ => panic!("Only working on linux right now..."),
    };
}

fn get_config_path() -> (PathBuf, PathBuf) {
    let mut config_dir: PathBuf = match var("HOME") {
        Ok(v) => PathBuf::from(v),
        Err(_) => panic!("error at getting $HOME"),
    };
    config_dir = config_dir.join(".config");

    // if the directory does not exist create one
    if !config_dir.join("terminalviva").exists() {
        match std::fs::create_dir(config_dir.join("terminalviva")) {
            Ok(_) => (),
            Err(e) => panic!("error at creating configuration directory: {}", e),
        };
    }
    config_dir = config_dir.join("terminalviva");

    (
        config_dir.join(".credentials.json").to_owned(),
        config_dir.join("config.toml"),
    )
}

fn get_raw_config() -> Config {
    allowed_linux();
    let paths = get_config_path();
    // get config instance from config.toml
    let config = match Config::builder()
        .add_source(File::with_name(paths.1.to_str().unwrap()))
        .build()
    {
        Ok(v) => v,
        Err(e) => panic!("error reading config.toml: {}", e),
    };
    config
}

fn get_user_settings(config: &Config) -> ConfigSettings {
    let config_settings: ConfigSettings = match config.get("settings") {
        Ok(v) => v,
        Err(e) => panic!("error at parsing config settings: {}", e),
    };

    config_settings
}

fn get_default_headers(config: &Config) -> HeaderMap {
    // get default headers
    let mut headers = HeaderMap::new();

    // get headers from config.toml
    let raw_default_headers: Vec<DefaultHeaders> = match config.get("headers") {
        Ok(v) => v,
        Err(e) => panic!("error at parsing default_headers: {}", e),
    };

    // convert raw_default_headers to HeaderMap
    for pair in raw_default_headers {
        let value: HeaderValue = match pair.value.parse() {
            Err(_) => continue,
            Ok(v) => v,
        };

        // convert key from &str to &'static str
        let key: &'static str = unsafe { std::mem::transmute(pair.key.as_bytes()) };
        headers.insert(key, value);
    }

    headers
}

pub fn get_config() -> UserConfig {
    let config = get_raw_config();
    let default_headers = get_default_headers(&config);
    let user_settings = get_user_settings(&config);

    UserConfig {
        raw_body: config,
        paths: get_config_path(),
        default_headers,
        user_settings,
    }
}

// get token from .credentials.json file
pub fn get_token() -> Mutex<Option<TokenCredential>> {
    let file = match std::fs::File::open(".credentials.json") {
        Ok(v) => v,
        Err(_) => return Mutex::new(None),
    };

    let token_credentials: TokenCredential = match serde_json::from_reader(file) {
        Ok(v) => v,
        Err(_) => return Mutex::new(None),
    };

    Mutex::new(Some(token_credentials))
}
