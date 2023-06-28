// Module: api
use std::io::Write;
use std::path::Path;
use crate::{CONFIG, DEFAULT_HEADERS, TOKEN};
use crate::response_types::*;

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
    pub studentId: String
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
            studentId: String::new()
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

    raw_result.text().await.expect("Error at converting response to text")
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
        studentId: String::new()
    };
    // TODO: WATCH THE TERMINAL OUTPUT AND BUILD A STRUCT OVER IT TO HANDLE THE 5 TIMES UNMATCHED CREDENTIAL

    match raw_result.json::<LoginResponse>().await {
        Ok(res) => {
            match res {
                LoginResponse::LoginPayload(v) => {
                    // println!("token: {}", v.token);
                    // // expired Token bZy34ISgtlBPfZ+CU48rWJGT6+6WZrOiI711xrjuFI7+eaQNfK1QFSD3Uj3/ceuDMwWhLu+nGA0ugSVhPfphOr/5kvtIJ5FgDAbzKFqzi824TP6HxFPZ2bJeDhg3uzin6OU0Aht4/vTpBQ5tQSqNZN36F05MTgkiW9er3wsGg56hFA8CKhPbgSz1aptdgSeTANrKwr54xFl7d20a+NIg5arv0gMxR9s/pzn2UMvyCf94JxlWKC4Ld0/7IXxeO1T6Zr4/dCNxTJayQxhPO8eapJCKSWDXeD6fYwJxDd0ltmH3dC2+0M73bLNCV8r7ZnFOCDdOoyzApUZYErOS1xT5loTp2qvlIv7tIN5Pa2gyicV7MDRawr9UfcyaTC/ZR1zdpV7elMHF11bH5Vc71CSe6g==
                    // println!("tokenAP: {}", v.tokenAP);
                    // // expired TokenAP 8zOV/SqmpGRpQ3kzBqrl3GqFzmQC4U2L4IwoD84UI70N8PBpEhfqqCA4PmVZ8nWeeCcZViguC3dP+yF8XjtU+m5MtPQV8Hb0UPfR2YqjLpJZbUyyDgeaOqpnFRJRnISgMUAGQ8d+J6Q/nvY4M5XW0FZaNQRZLDH/UmEK14wwyWNKvbk6OWjTGl/0Hrfyeo4vQ3rtsqtcr5T/cS7cQ/XZtJPu8UcEmpwYZ0ArpAGOo7zmepxYGnE44mi+dUQ0Ylj+a3kzkyUXXpN8HdW8CQneME+XMeZSfYMKJcmbFsNXIl14ypPM++NicCSVOiVzWwxuPSgXbBNnTj5Qn01Nsfy36oYeZ86b0Kbrr91ts/VHBeWHDSuLPZ88u4lp9eBHS4YN9bqlAlB+vx4IjAka2yIboQ==
                    token_credential.token = v.token;
                    token_credential.tokenAP = v.tokenAP;
                    // remove the first and the last character from the ident field to obtain the studentId
                    token_credential.studentId = v.ident[1..v.ident.len()-1].to_string();

                    update_token(&token_credential);
                },
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

pub async fn absences_request() -> String {
    let url = format!("{}/students/<studentID>/absences/details", BASE_URL);
    get_request(&url).await
}

pub async fn grades_request() -> Grades {
    let url = format!("{}/students/<studentID>/grades", BASE_URL);
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
                    },
                    ResponseResult::Grades(payload) => {
                        result = payload;
                        break;
                    },
                }
            },
            Err(e) => {
                panic!("[ERROR]: Parsing grades response: {}", e)
            }
        };
    }
    result
}
