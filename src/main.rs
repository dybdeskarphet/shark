use std::io;
use std::io::Write;
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

fn execute(input: String) {
    let input_str = input.as_str();
    match input_str.trim() {
        "exit" => std::process::exit(0),
        "help" => {
            println!("Hello, this is Shark, a custom shell without a purpose.");
        }
        _ => command_parse_execute(input),
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
