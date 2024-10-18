use std::env;
use std::path::Path;

fn change_directory(input: &str) {
    let path = Path::new(input);
    let current_dir_status = env::set_current_dir(&path);
    match current_dir_status {
        Ok(_) => (),
        Err(e) => println!("Cannot change directory: {}", e),
    }
}
