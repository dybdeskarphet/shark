use crate::input_parser::parse;
use crate::shell_command::ShellCommand;
use std::env;
use std::path::Path;

pub fn run_builtin(full_input: &ShellCommand) -> Option<()> {
    match full_input.command.as_str() {
        "cd" => Some(change_directory(full_input.args.clone())),
        _ => None,
    }
}

fn get_current_dir() -> String {
    let current_dir = env::current_dir();

    match current_dir {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => {
            println!("shark: current dir doesn't exist or you don't have the permission to use it");
            "/".to_string()
        }
    }
}

fn change_directory(input: Option<Vec<String>>) {
    match input {
        Some(path) => {
            let current_dir = get_current_dir();
            let new_path = match path.get(0).to_owned() {
                Some(p) => Path::new(p),
                None => {
                    println!(
                        "shark: path argument flew away somewhere in the middle of your command"
                    );
                    Path::new(&current_dir)
                }
            };

            let change_dir_status = env::set_current_dir(new_path);

            match change_dir_status {
                Ok(_) => (),
                Err(e) => println!("shark: cannot change directory: {}", e),
            }
        }
        None => {
            println!("shark: cd needs a path");
        }
    }
}
