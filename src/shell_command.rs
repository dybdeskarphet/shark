use crate::builtin_commands::run_builtin;
use crate::input_parser::parse;
use std::process::Command;

pub struct ShellCommand {
    pub command: String,
    pub args: Option<Vec<String>>,
}

impl ShellCommand {
    pub fn create(input: String) -> Self {
        let (command, args) = parse(input);
        Self { command, args }
    }

    fn external_command(&self) {
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

    pub fn run(&self) {
        match run_builtin(self) {
            Some(_) => (),
            None => self.external_command(),
        }
    }
}
