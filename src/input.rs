use crate::api;
use crate::display;


#[derive(Debug)]
pub struct InputSource {
    pub command: String,
    pub args: Vec<String>,
}

impl InputSource {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<InputSource, &'static str> {
        args.next();
        let command = if let Some(value) = args.next() {
            value
        } else {
            return Err("no command provided");
        };

        let args: Vec<String> = args.collect();

        Ok(InputSource { command, args })
    }

    pub async fn process_command(&self) {
        match self.command.as_str() {
            "login" => {
                api::login().await;
            },
            "absence" => {
                let result = api::absences_request().await;
                println!("{result}");
            },
            "grade" => {
                let result = api::grades_request().await;
                let result = display::display_grades(result);
                println!("{}", result);

            },
            _ => println!("command not found"),
        };
    }
}
