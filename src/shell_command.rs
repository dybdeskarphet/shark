use crate::builtin_commands::run_builtin;
use crate::input_parser::parse;
use nix::sys::signal::kill;
use nix::sys::signal::SIGINT;
use nix::unistd::Pid;
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
        let child;

        match &self.args {
            Some(args) => {
                let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                child = Command::new(&self.command).args(args_ref).spawn();
            }
            None => {
                child = Command::new(&self.command).spawn();
            }
        }

        match child {
            Ok(child_process) => {
                let result = child_process.wait_with_output();
                match result {
                    Ok(_) => (),
                    Err(e) => print!("shark: couldn't wait for the process: {}", e),
                }
            }
            Err(e) => {
                println!("shark: cannot run the command: {}", e);
            }
        }
    }

    pub fn run(&self) {
        match run_builtin(self) {
            Some(_) => (),
            None => self.external_command(),
        }
    }
}
