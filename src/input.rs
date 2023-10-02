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
                // validate an iso 8601 string
                match chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                    Ok(date) => Some(date.to_string().replace("-", "")),
                    Err(_) => {
                        panic!("Invalid date format, please follow ISO 8601 standard format(YYYY-MM-DD)");
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
            let result = display::display_grades(result, grade_settings);
            println!("{}", result);
        }
        Commands::Agenda => {
            let agenda_settings = AgendaSettings::new(settings, args.date);
            let result = api::agenda_request(agenda_settings.date).await;
            let result = display::display_agenda(result);
            println!("{}", result);
        }
        Commands::Test => {
            println!("Test");
        }
    }
}
