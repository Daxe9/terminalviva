use crate::input::GradeSettings;
use crate::response_types::*;
use crate::CONFIG_SETTINGS;
use chrono::{DateTime, FixedOffset};
use tabled::{
    settings::{object::Rows, Alignment, Modify, Style, Width},
    Table, Tabled,
};

trait DefaultStyle {
    fn add_default_style(&mut self);
}

impl DefaultStyle for Table {
    fn add_default_style(&mut self) {
        self.with(Style::modern())
            // align the first row to the center
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(
                Modify::new(Rows::new(1..))
                    .with(Width::wrap(CONFIG_SETTINGS.wrap_width).keep_words()),
            );
    }
}

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

    if simplified_grades.is_empty() {
        return String::from("No records");
    }

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
    table.add_default_style();

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
            _ => "",
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

    if simplified_absences.is_empty() {
        return String::from("No records");
    }

    let mut table = Table::new(simplified_absences);
    table.add_default_style();

    table.to_string()
}

#[allow(non_snake_case)]
#[derive(Tabled)]
struct SimpleAgenda {
    #[tabled(skip)]
    time: DateTime<FixedOffset>,
    date: String,
    code: String,
    notes: String,
    teacher: String,
}

impl SimpleAgenda {
    fn from_agenda(agenda: Agenda) -> Self {
        let processed_time =
            DateTime::parse_from_str(&agenda.evtDatetimeBegin, "%Y-%m-%dT%H:%M:%S%z").unwrap();
        SimpleAgenda {
            time: processed_time,
            code: match &agenda.evtCode[..] {
                "AGHW" => "Homework".to_string(),
                "AGNT" => "Nota".to_string(),
                _ => agenda.evtCode,
            },
            teacher: agenda.authorName,
            notes: agenda.notes,
            date: "".to_string(),
        }
    }
}

pub fn display_agenda(agenda: Agendas) -> String {
    // TODO: sort the agenda by date
    let mut simplified_agenda: Vec<SimpleAgenda> = agenda
        .agenda
        .into_iter()
        .map(SimpleAgenda::from_agenda)
        .collect();

    if simplified_agenda.is_empty() {
        return String::from("No records");
    }

    simplified_agenda.sort_by(|a, b| a.time.cmp(&b.time));

    for record in simplified_agenda.iter_mut() {
        record.date = record.time.format("%Y-%m-%d %A").to_string();
    }
    // simplified_agenda.iter_mut().map(|mut a| {
    //     // format the date to yyyy-mm-dd weekday
    //     let temp = DateTime::parse_from_str(&a.time.to_string(), "%Y-%m-%d %A").unwrap();
    //     a.time = temp;
    //     a
    // });
    //.horizontals([HorizontalLine::new(2, Style::rounded().get_horizontal())]);

    let mut table = Table::new(simplified_agenda);
    table.add_default_style();

    table.to_string()
}
