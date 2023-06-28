use config::{Config, File};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use crate::api::TokenCredential;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct DefaultHeaders {
    pub key: String,
    pub value: String,
}

pub fn get_config() -> Option<Config> {
    // get config instance from config.toml
    let config = match Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
    {
        Ok(v) => v,
        Err(e) => panic!("error parsing config.toml: {}", e),
    };
    Some(config)
}

pub fn get_default_headers() -> HeaderMap {
    // get default headers
    let mut headers = HeaderMap::new();

    // get headers from config.toml
    let raw_default_headers: Vec<DefaultHeaders> =
        match crate::CONFIG.as_ref().unwrap().get("headers") {
            Ok(v) => v,
            Err(e) => panic!("error parsing default_headers: {}", e),
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

pub fn get_token() -> Mutex<Option<TokenCredential>> {
    let file = match std::fs::File::open(".credentials.json") {
        Ok(v) => v,
        Err(_) => return Mutex::new(None),
    };

    let token_credentials: TokenCredential = match serde_json::from_reader(file) {
        Ok(v) => v,
        Err(_) => return Mutex::new(None)
    };

    Mutex::new(Some(token_credentials))
}
