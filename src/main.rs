use std::io;
use std::io::Write;

fn execute(input: String) {
    let input_str = input.as_str();
    match input_str.trim() {
        "exit" => std::process::exit(0),
        "help" => {
            println!("Hello, this is Shark, a custom shell without a purpose.");
        }
        _ => print!("{}", input),
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
