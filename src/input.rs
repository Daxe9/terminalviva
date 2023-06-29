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
        // when provided set this value to true
        default_value = "false"
        )]
    desc_date: bool
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "grade", about = "Display grades of the current user")]
    Grade,
    #[clap(name = "absence", about = "Display absences of the current user")]
    Absence,
    #[clap(name = "login", about = "Login to spaggiari")]  
    Login
}

pub struct Settings {
    pub desc_date: bool,
}

impl Settings {
    fn new(desc_date: bool) -> Self {
        Settings { desc_date }
    }
}

pub async fn process_input() {
    let args = Args::parse();

    let settings = Settings::new(args.desc_date);
    
    match args.command {
        Commands::Login => {
            api::login().await;
        },
        Commands::Absence => {
            let result = api::absences_request().await;
            println!("{result}");
        },
        Commands::Grade => {
            let result = api::grades_request().await;
            let result = display::display_grades(result, settings);
            println!("{}", result);
        },
    }

}

// impl InputSource {
//     pub fn build(mut args: impl Iterator<Item = String>) -> Result<InputSource, &'static str> {
//         args.next();
//         let command = if let Some(value) = args.next() {
//             value
//         } else {
//             return Err("no command provided");
//         };

//         let args: Vec<String> = args.collect();

//         Ok(InputSource { command, args })
//     }

//     pub async fn process_command(&self) {
//         match self.command.as_str() {
//             "login" => {
//                 api::login().await;
//             },
//             "absence" => {
//                 let result = api::absences_request().await;
//                 println!("{result}");
//             },
//             "grade" => {
//                 let result = api::grades_request().await;



//                 let result = display::display_grades(result);
//                 println!("{}", result);

//             },
//             _ => println!("command not found"),
//         };
//     }
// }
