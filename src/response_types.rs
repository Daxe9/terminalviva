// disable snake case warning for this file since these types are parsed from an external server
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ResponseResult {
    ExpiredToken(ExpiredToken),
    Grades(Grades),
    Absences(Absences),
    Agendas(Agendas),
    Lessons(Lessons),
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

impl Grades {
    pub fn new() -> Self {
        Grades { grades: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grade {
    pub subjectId: u32,
    pub subjectCode: String,
    pub subjectDesc: String,
    pub evtId: u32,
    pub evtDate: String,
    pub decimalValue: f64,
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

    pub weightFactor: f64,

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Absences {
    pub events: Vec<Absence>,
}

impl Absences {
    pub fn new() -> Self {
        Absences { events: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Absence {
    pub evtId: u32,
    // TODO: find out each code's meaning
    pub evtCode: String,
    pub evtDate: String,
    pub evtHPos: Option<u32>,
    pub evtValue: Option<u32>,
    pub isJustified: bool,
    // TODO: find out each code's meaning
    pub justifReasonCode: Option<String>,
    pub justifReasonDesc: Option<String>,
    // NOTE: The type of the vec is still unknown
    pub hoursAbsence: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agendas {
    pub agenda: Vec<Agenda>,
}

impl Agendas {
    pub fn new() -> Self {
        Agendas { agenda: Vec::new() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Agenda {
    pub evtId: u32,
    // TODO: find out each code's meaning
    pub evtCode: String,
    pub evtDatetimeBegin: String,
    pub evtDatetimeEnd: String,
    pub isFullDay: bool,
    pub notes: String,
    pub authorName: String,
    pub classDesc: String,
    pub subjectId: Option<u32>,
    pub subjectDesc: Option<String>,
    pub homeworkId: Option<Value>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Lessons {
    pub lessons: Vec<Lesson>,
}

impl Lessons {
    pub fn new() -> Self {
        Lessons {
            lessons: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lesson {
    pub evtId: u32,
    pub evtDate: String,
    pub evtCode: String,
    pub evtHPos: u32,
    pub evtDuration: u32,
    pub classDesc: String,
    pub authorName: String,
    pub subjectId: Option<u32>,
    pub subjectCode: Option<Value>,
    pub subjectDesc: Option<String>,
    pub lessonType: Option<String>,
    pub lessonArg: String,
}
