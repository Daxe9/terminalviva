use crate::api;
use crate::display;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Davide Xie", version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(
        short,
        long,
        global = true,
        help = "Display date in descending order",
        default_value = "false"
    )]
    desc_date: bool,

    #[arg(short, long, global = true, help = "Select a specific subject")]
    name: Option<String>,
    #[arg(
        long,
        global = true,
        help = "Select a specific date(ISO 8601, YYYY-MM-DD)"
    )]
    date: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "grade", about = "Display grades of the current user")]
    Grade,
    #[clap(name = "lesson", about = "Display current week lessons")]
    Lesson,
    #[clap(name = "absence", about = "Display absences of the current user")]
    Absence,
    #[clap(name = "login", about = "Login to spaggiari")]
    Login,
    #[clap(
        name = "agenda",
        about = "Display agenda of the current user, default is the current day"
    )]
    Agenda,
    Test,
}

pub struct Settings {
    pub desc_date: bool,
}

impl Settings {
    fn new(desc_date: bool) -> Self {
        Settings { desc_date }
    }
}
pub struct GradeSettings {
    pub settings: Settings,
    pub name: Option<String>,
}

impl GradeSettings {
    fn new(settings: Settings, name: Option<String>) -> Self {
        GradeSettings { settings, name }
    }
}

pub struct AgendaSettings {
    pub settings: Settings,
    pub date: Option<String>,
}

impl AgendaSettings {
    fn new(settings: Settings, date: Option<String>) -> Self {
        let date = match date {
            Some(date) => {
                if date == "nextweek" {
                    Some(date)
                } else {
                    // validate an iso 8601 string
                    match chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                        Ok(date) => Some(date.to_string().replace("-", "")),
                        Err(_) => {
                            panic!("Invalid date format, please follow ISO 8601 standard format(YYYY-MM-DD)");
                        }
                    }
                }
            }
            None => None,
        };
        AgendaSettings { settings, date }
    }
}

pub async fn process_input() {
    let args = Args::parse();

    let settings = Settings::new(args.desc_date);

    match args.command {
        Commands::Login => {
            api::login().await;
        }
        Commands::Absence => {
            let result = api::absences_request().await;
            let result = display::display_absences(result);
            println!("{}", result);
        }
        Commands::Grade => {
            let grade_settings = GradeSettings::new(settings, args.name);
            let result = api::grades_request().await;
            let (result, grades) = display::display_grades(result, grade_settings);

            // average
            let average = grades.iter().fold(0.0, |a, b| a + b.grade) / (grades.len() as f64);

            // weighted average
            let mut sum: f64 = 0.0;
            let mut weights: f64 = 0.0;
            for grade in grades {
                sum += grade.grade * grade.weight;
                weights += grade.weight;
            }
            let weighted_average = sum / weights;

            println!("{}", result);
            println!("The average grade is {:.2}.", average);
            println!("The weighted average grade is {:.2}.", weighted_average);
        }
        Commands::Agenda => {
            let agenda_settings = AgendaSettings::new(settings, args.date);
            let result = api::agenda_request(agenda_settings.date).await;
            let result = display::display_agenda(result);
            println!("{}", result);
        }
        Commands::Lesson => {
            let lesson_settings = AgendaSettings::new(settings, args.date);
            let result = api::lessons_request(lesson_settings.date).await;
            let result = display::display_lessons(result);
            println!("{}", result);
        }
        Commands::Test => {
            println!("Test");
        }
    }
}
