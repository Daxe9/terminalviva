// NOTE: Structs prefixed with U are unused structs, however they are necessary for the deserialization of the JSON response

// disable snake case warning for this file
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ResponseResult {
    ExpiredToken(ExpiredToken),
    Grades(Grades),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExpiredToken {
    pub statusCode: u16,
    pub error: String,
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Grades {
    pub grades: Vec<Grade>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grade {
    pub subjectId: u32,
    pub subjectCode: String,
    pub subjectDesc: String,
    pub evtId: u32,
    pub evtDate: String,
    pub decimalValue: f32,
    pub displayValue: String,
    // The misspelling of this field is not an error. All the decision is made by the server
    pub displaPos: u32,
    pub notesForFamily: String,
    pub color: String,
    pub canceled: bool,
    pub underlined: bool,
    pub periodPos: u32,
    pub periodDesc: String,
    pub componentPos: u32,
    pub componentDesc: String,

    pub weightFactor: f32,

    pub skillId: u32,
    pub gradeMasterId: u32,
    pub skillDesc: Option<Value>,
    pub skillCode: Option<Value>,
    pub skillMasterId: u32,
    pub skillValueDesc: String,
    pub skillValueShortDesc: Option<Value>,
    // It's not an error if this field has the s in lowercase. All the decision is made by the server
    pub oldskillId: u32,
    pub oldskillDesc: String,
}

impl Grades {
    pub fn new() -> Self {
        Grades { grades: Vec::new() }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoginResponse {
    LoginPayload(LoginPayload),
    LoginError(LoginError),
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub expire: String,
    pub firstName: String,
    pub ident: String,
    pub lastName: String,
    pub release: String,
    pub showPwdChangeReminder: bool,
    pub token: String,
    pub tokenAP: String,
    // remove the first and last character from the ident field to obtain this value
    // pub studentId: String
}

#[derive(Deserialize, Debug)]
pub struct LoginError {
    pub statusCode: u16,
    pub error: String,
    pub info: String,
    pub message: String,
}
