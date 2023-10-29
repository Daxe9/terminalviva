use crate::api::TokenCredential;
use config::{Config, File};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct DefaultHeaders {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSettings {
    #[serde(alias = "wrap-width")]
    pub wrap_width: usize,
    #[serde(alias = "credentials-file-path")]
    pub credentials_file_path: String,
}

pub struct UserConfig {
    pub raw_body: Config,
    pub default_headers: HeaderMap,
    pub user_settings: ConfigSettings,

}

fn get_raw_config() -> Config {
    // get config instance from config.toml
    let config = match Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
    {
        Ok(v) => v,
        Err(e) => panic!("error parsing config.toml: {}", e),
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
    let raw_default_headers: Vec<DefaultHeaders> =
        match config.get("headers") {
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
        default_headers,
        user_settings
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
