use crate::response_types::*;
use tabled::{Table, Tabled, settings::Style};


#[allow(non_snake_case)]
#[derive(Tabled)]
struct SimpleGrade {
    subject: String,
    date: String,
    value: f32,
    subjectType: String,
    weightFactor: f32,
}

impl SimpleGrade {
    fn from_grade(grade: Grade) -> Self {
        SimpleGrade {
            subject: grade.subjectDesc,
            date: grade.evtDate,
            value: grade.decimalValue,
            subjectType: grade.componentDesc,
            weightFactor: grade.weightFactor,
        }
    }
}

pub fn display_grades(grades: Grades) -> String {
    let simplified_grades: Vec<SimpleGrade> = grades.grades.into_iter().map(|grade| SimpleGrade::from_grade(grade)).collect();

    let mut table = Table::new(&simplified_grades);
    table.with(Style::rounded());

    let result = table.to_string();

    result
}