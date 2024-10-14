use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn command_parse_execute(command: String) {
    let command_output = if command.trim().contains(char::is_whitespace) {
        let (program, args) = command
            .split_once(char::is_whitespace)
            .expect("Something went wrong while parsing the command");
        let mut args: Vec<&str> = args.split(char::is_whitespace).collect();

        if let Some(element) = args.last() {
            if *element == "" {
                args.pop();
            }
        }

        Command::new(program).args(&args).status()
    } else {
        Command::new(command.trim()).status()
    };

    match &command_output {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn change_directory(input: &str) {
    let path = Path::new(input);
    let current_dir_status = env::set_current_dir(&path);
    match current_dir_status {
        Ok(_) => (),
        Err(e) => println!("Cannot change directory: {}", e),
    }
}

fn execute(input: String) {
    let input_vec: Vec<&str>;

    if input.trim().contains(char::is_whitespace) {
        input_vec = input.trim().split_whitespace().collect();
    } else {
        input_vec = vec![&input.trim()];
    }

    if input_vec.len() > 1 {
        match input_vec.get(0) {
            Some(&"cd") => {
                change_directory(input_vec.get(1).expect("Cannot parse the args for `cd`"))
            }
            _ => command_parse_execute(input),
        }
    } else {
        match input_vec.get(0) {
            Some(&"help") => println!("Hey!"),
            _ => {
                let command_stripped = input_vec
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_else(String::new)
                    .trim()
                    .to_string();
                command_parse_execute(command_stripped);
            }
        };
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(".");
        execute(input);
    }
}
