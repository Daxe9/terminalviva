// Module: api
use crate::response_types::*;
use crate::{CONFIG, DEFAULT_HEADERS, TOKEN};
use std::io::Write;
use std::path::Path;
use chrono::prelude::Utc;

use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://web.spaggiari.eu/rest/v1";
#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct TokenCredential {
    pub token: String,
    pub tokenAP: String,
    pub studentId: String,
}

#[derive(Serialize)]
pub struct LoginData {
    pub ident: String,
    pub uid: String,
    pub pass: String,
}

impl LoginData {
    pub fn new(credentials: Credentials) -> LoginData {
        LoginData {
            ident: credentials.username.to_owned(),
            uid: credentials.username,
            pass: credentials.password,
        }
    }
}

/*
   Check whether the file exists or not
   if it exists, remove it in order to create a new one with updated token
   Otherwise, create a new one with the token
   */
fn update_token(token_credential: &TokenCredential) {
    let file_path = Path::new(".credentials.json");
    let existing_token_file = file_path.exists();

    if existing_token_file {
        std::fs::remove_file(".credentials.json").expect("Error at removing file");
    }

    // create file
    let mut file = std::fs::File::create(".credentials.json").expect("Error at creating file");
    // serialize token_credential to json
    let token_credential_json = serde_json::to_string(&token_credential).unwrap();
    // write json to file
    file.write_all(token_credential_json.as_bytes()).unwrap();
}

async fn get_request(url: &str) -> String {
    let mut token_credential = match TOKEN.lock().unwrap().as_ref() {
        Some(v) => v.clone(),
        None => TokenCredential {
            token: String::new(),
            tokenAP: String::new(),
            studentId: String::new(),
        },
    };

    // if token is not present, login
    if let Some(v) = foreplay().await {
        token_credential = v;
    }

    // process url with studentId
    let url = process_url(url.to_owned(), &token_credential.studentId);

    let client = reqwest::Client::new();
    let raw_result = match client
        .get(&url)
        .headers(DEFAULT_HEADERS.to_owned())
        .header("z-auth-token", token_credential.token.as_str())
        .send()
        .await
        {
            Ok(v) => v,
            Err(e) => panic!("error sending get request at {}: {}", url, e),
        };

    raw_result
        .text()
        .await
        .expect("Error at converting response to text")
}

fn process_url(url: String, student_id: &str) -> String {
    url.replace("<studentID>", student_id)
}

// beautiful name
async fn foreplay() -> Option<TokenCredential> {
    // check if token is present or not
    if TOKEN.lock().unwrap().as_ref().is_none() {
        println!("Re-login...");
        let token_credential = login().await;
        return Some(token_credential);
    }
    None
}

pub async fn login() -> TokenCredential {
    // get credentials
    let credentials: Credentials = match CONFIG.as_ref().unwrap().get("credentials") {
        Ok(v) => v,
        Err(e) => panic!("error parsing credentials: {}", e),
    };

    let login_data = LoginData::new(credentials);

    let client = reqwest::Client::new();
    let raw_result = match client
        .post(format!("{}/auth/login", BASE_URL))
        .headers(DEFAULT_HEADERS.to_owned())
        .json(&login_data)
        .send()
        .await
        {
            Ok(v) => v,
            Err(e) => panic!("error sending login request: {}", e),
        };
    let mut token_credential = TokenCredential {
        token: String::new(),
        tokenAP: String::new(),
        studentId: String::new(),
    };

    match raw_result.json::<LoginResponse>().await {
        Ok(res) => {
            match res {
                LoginResponse::LoginPayload(v) => {
                    token_credential.token = v.token;
                    token_credential.tokenAP = v.tokenAP;
                    // remove the first and the last character from the ident field to obtain the studentId
                    token_credential.studentId = v.ident[1..v.ident.len() - 1].to_string();

                    update_token(&token_credential);
                }
                LoginResponse::LoginError(info) => {
                    eprintln!("Login failed: {}", info.message);
                    // check if a file exists or not
                    if Path::new("./.credentials.json").exists() {
                        // remove the token by removing .credentials.json file
                        match std::fs::remove_file("./.credentials.json") {
                            Ok(_) => (),
                            Err(_) => eprintln!("[IMPORTANT]: Failed to remove .credentials.json file, please remove it manually [~/.credentials.json]")
                        };
                    }
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            // remove the token by removing .credentials.json file
            panic!("[ERROR]: Parsing JSON {}", e);
        }
    };

    token_credential
}

pub async fn absences_request() -> Absences {
    let url = format!("{}/students/<studentID>/absences/details", BASE_URL);
    // disable the warning for unused_assignments
    #[allow(unused_assignments)]
    let mut result = Absences::new();
    loop {
        let raw_result = get_request(&url).await;

        match serde_json::from_str(&raw_result) {
            Ok(response) => {
                match response {
                    ResponseResult::ExpiredToken(_) => {
                        println!("Re-login...");
                        // Re-login
                        let token_credential = login().await;

                        // replace the token
                        TOKEN.lock().unwrap().replace(token_credential);
                    }
                    ResponseResult::Absences(payload) => {
                        result = payload;
                        break;
                    }
                    _ => {
                        panic!("[ERROR]: Wrong return type upon api call {}", raw_result)
                    }
                }
            }
            Err(e) => {
                panic!("[ERROR]: Parsing absence response: {}", e)
            }
        };
    }
    result
}

pub async fn grades_request() -> Grades {
    let url = format!("{}/students/<studentID>/grades", BASE_URL);
    // disable the warning for unused_assignments
    #[allow(unused_assignments)]
    let mut result = Grades::new();
    loop {
        let raw_result = get_request(&url).await;

        match serde_json::from_str(&raw_result) {
            Ok(response) => {
                match response {
                    ResponseResult::ExpiredToken(_) => {
                        println!("Re-login...");
                        // Re-login
                        let token_credential = login().await;

                        // replace the token
                        TOKEN.lock().unwrap().replace(token_credential);
                    }
                    ResponseResult::Grades(payload) => {
                        result = payload;
                        break;
                    }

                    _ => {
                        panic!("[ERROR]: Wrong return type upon api call {}", raw_result)
                    }
                }
            }
            Err(e) => {
                panic!("[ERROR]: Parsing grades response: {}", e)
            }
        };
    }
    result
}

// The default behavior should be fetching the agenda of the current day
pub async fn agenda_request(selected_date: Option<String>) -> Agendas {
    // get a date in format like this 20230901

    let date = if selected_date.is_none() {
        let temp = Utc::now();
        temp.format("%Y%m%d").to_string()
    } else {
        selected_date.unwrap()
    };

    // make the url
    let url = format!("{}/students/<studentID>/agenda/all/{}/{}", BASE_URL, &date, date);

    let mut result = Agendas::new();
    loop {
        let raw_result = get_request(&url).await;

        match serde_json::from_str(&raw_result) {
            Ok(response) => {
                match response {
                    ResponseResult::ExpiredToken(_) => {
                        println!("Re-login...");
                        // Re-login
                        let token_credential = login().await;

                        // replace the token
                        TOKEN.lock().unwrap().replace(token_credential);
                    }
                    ResponseResult::Agendas(payload) => {
                        result = payload;
                        break;
                    }

                    _ => {
                        panic!("[ERROR]: Wrong return type upon api call {}", raw_result)
                    }
                }
            }
            Err(e) => {
                panic!("[ERROR]: Parsing grades response: {}", e)
            }
        };
    }
    result

}
