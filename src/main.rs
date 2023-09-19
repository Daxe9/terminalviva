mod api;
mod display;
mod input;
mod response_types;
mod settings;
use config::Config;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref DEFAULT_HEADERS: HeaderMap = settings::get_default_headers();
    pub static ref CONFIG: Option<Config> = settings::get_config();
    // this is made with mutex, however, this is single-threaded.
    // The only purpose of mutex in this case is to make it mutable.
    pub static ref TOKEN: Mutex<Option<api::TokenCredential>> = settings::get_token();
}

#[tokio::main]
async fn main() {
    input::process_input().await;
}
