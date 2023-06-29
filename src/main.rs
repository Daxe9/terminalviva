mod api;
mod input;
mod settings;
mod response_types;
mod display;
use config::Config;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref DEFAULT_HEADERS: HeaderMap = settings::get_default_headers();
    pub static ref CONFIG: Option<Config> = settings::get_config();
    pub static ref TOKEN: Mutex<Option<api::TokenCredential>> = settings::get_token();
}

#[tokio::main]
async fn main() {
    input::process_input().await;
}
