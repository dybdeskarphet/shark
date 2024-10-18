use std::process::Command;

pub struct ShellCommand {
    command: String,
    args: Option<Vec<String>>,
}

impl ShellCommand {
    fn parse(command: String) -> (String, Option<Vec<String>>) {
        if command.trim().contains(char::is_whitespace) {
            let (program, args) = command
                .split_once(char::is_whitespace)
                .expect("Something went wrong while parsing the command");

            let mut args_vec: Option<Vec<String>> =
                Some(args.split_whitespace().map(String::from).collect());
            let program_string = program.to_string();

            if let Some(vec) = &mut args_vec {
                if let Some(last) = vec.last() {
                    if last.is_empty() {
                        vec.pop();
                    }
                }
            }

            (program_string, args_vec)
        } else {
            (command.trim().to_string(), None)
        }
    }

    pub fn create(input: String) -> Self {
        let (command, args) = Self::parse(input);
        Self { command, args }
    }

    pub fn run(&self) {
        let output;
        match &self.args {
            Some(args) => {
                let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                output = Command::new(&self.command).args(args_ref).status();
            }
            None => {
                output = Command::new(&self.command).status();
            }
        }

        match output {
            Ok(_) => (),
            Err(e) => println!("shark: {}", e),
        }
    }
}
