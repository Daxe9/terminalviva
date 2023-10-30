mod api;
mod display;
mod input;
mod response_types;
mod settings;
use lazy_static::lazy_static;
use settings::*;
use std::sync::Mutex;

lazy_static! {
    pub static ref USER_CONFIG: UserConfig = get_config();
    // this is made with mutex, however, this is single-threaded.
    // The only purpose of mutex in this case is to make it mutable.
    pub static ref TOKEN: Mutex<Option<api::TokenCredential>> = get_token();
}

#[tokio::main]
async fn main() {
    input::process_input().await;
}
