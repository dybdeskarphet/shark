use std::io;
use std::io::Write;
mod builtin_commands;
mod input_parser;
mod shell_command;
use shell_command::ShellCommand;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).expect(".");
        if bytes_read == 0 { /* EOF without any command. Exit */ break; }
        let child = ShellCommand::create(input);
        child.run();
    }
}
