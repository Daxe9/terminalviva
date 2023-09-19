use crate::input::GradeSettings;
use crate::response_types::*;
use tabled::settings::Alignment;
use tabled::{
    settings::{object::Rows, Modify, Style},
    Table, Tabled,
};

#[allow(non_snake_case)]
#[derive(Tabled)]
struct SimpleGrade {
    subject: String,
    date: String,
    grade: f32,
    subject_type: String,
    weight: f32,
}

impl SimpleGrade {
    fn from_grade(grade: Grade) -> Self {
        let mut subject_desc = grade.subjectDesc;
        if subject_desc.ends_with("sez. INFORMATICA") {
            subject_desc = subject_desc.replace("sez. INFORMATICA", "");
        }

        let subject = match &subject_desc[..] {
            "LINGUA E LETTERATURA ITALIANA" => "ITALIANO".to_string(),
            "STORIA,CITTADINANZA E COSTITUZIONE" => "STORIA".to_string(),
            "SCIENZE MOTORIE E SPORTIVE" => "MOTORIA".to_string(),
            "TECNOLOGIE E PROGETTAZIONE DI SISTEMI INFORMATICI E DI TELECOMUNICAZIONI " => {
                "TPSIT".to_string()
            }
            _ => subject_desc,
        };

        SimpleGrade {
            subject,
            date: grade.evtDate,
            grade: grade.decimalValue,
            subject_type: grade.componentDesc,
            weight: grade.weightFactor,
        }
    }
}

fn sort_date_grade(grades: Vec<SimpleGrade>, from_oldest_to_youngest: bool) -> Vec<SimpleGrade> {
    let mut grades = grades;
    grades.sort_by(|a, b| {
        // split the date value and parse three values to i32 and collect them into a vector
        let a_date: Vec<i32> = a
            .date
            .split('-')
            .map(|x| x.parse::<i32>().expect("[ERROR]: Cannot parse date object"))
            .collect();
        let b_date: Vec<i32> = b
            .date
            .split('-')
            .map(|x| x.parse::<i32>().expect("[ERROR]: Cannot parse date object"))
            .collect();

        // sum the values of the date vector
        let a_date = a_date[0] * 365 + a_date[1] * 30 + a_date[2];
        let b_date = b_date[0] * 365 + b_date[1] * 30 + b_date[2];

        if from_oldest_to_youngest {
            b_date.cmp(&a_date)
        } else {
            a_date.cmp(&b_date)
        }
    });

    grades
}

pub fn display_grades(grades: Grades, grade_settings: GradeSettings) -> String {
    let simplified_grades: Vec<SimpleGrade> = grades
        .grades
        .into_iter()
        .map(SimpleGrade::from_grade)
        .collect();

    // filter it by name if the name is specified
    let simplified_grades: Vec<SimpleGrade> = match grade_settings.name {
        Some(name) => simplified_grades
            .into_iter()
            .filter(|x| x.subject.to_lowercase().trim() == name.to_lowercase().trim())
            .collect(),
        None => simplified_grades,
    };

    let simplified_grades = sort_date_grade(simplified_grades, grade_settings.settings.desc_date);
    let mut table = Table::new(simplified_grades);
    table
        .with(Style::rounded())
        // align the first row to the center
        .with(Modify::new(Rows::first()).with(Alignment::center()));

    table.to_string()
}


#[allow(non_snake_case)]
#[derive(Tabled)]
struct SimpleAbsence {
    id: String,
    date: String,
    justified: bool,
    reason: String,
    code: String,
}

impl SimpleAbsence {
    fn from_absence(absence: Absence) -> Self {
        let (reason, code) = if absence.justifReasonDesc.is_none() {
            ("N/A".to_string(), "N/A".to_string())
        } else {
            (
                absence.justifReasonDesc.unwrap(),
                absence.justifReasonCode.unwrap(),
            )
        };

        let type_absence = match &absence.evtCode[..] {
            "ABA0" => "Assenza",
            "ABR0" => "Ritardo",
            "ABR1" => "R. Breve",
            _ => ""
        };

        SimpleAbsence {
            id: type_absence.to_string(),
            date: absence.evtDate,
            justified: absence.isJustified,
            reason,
            code,
        }
    }
}


pub fn display_absences(absences: Absences) -> String {
    let simplified_absences: Vec<SimpleAbsence> = absences
        .events
        .into_iter()
        .map(SimpleAbsence::from_absence)
        .collect();

    let mut table = Table::new(simplified_absences);
    table
        .with(Style::rounded())
        // align the first row to the center
        .with(Modify::new(Rows::first()).with(Alignment::center()));

    table.to_string()
}


#[allow(non_snake_case)]
#[derive(Tabled)]
struct SimpleAgenda {
    code: String,
    notes: String,
    teacher: String,
    class: String,
}

impl SimpleAgenda {
    fn from_agenda(agenda: Agenda) -> Self {
        SimpleAgenda {
            code: agenda.evtCode,
            teacher: agenda.authorName,
            class: agenda.classDesc,
            notes: agenda.notes
        }
    }
}

pub fn display_agenda(agenda: Agendas) -> String {
    let simplified_agenda: Vec<SimpleAgenda> = agenda
        .agenda
        .into_iter()
        .map(SimpleAgenda::from_agenda)
        .collect();

    let mut table = Table::new(simplified_agenda);
    table
        .with(Style::rounded())
        // align the first row to the center
        .with(Modify::new(Rows::first()).with(Alignment::center()));

    table.to_string()
}
