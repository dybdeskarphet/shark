pub fn parse(command: String) -> (String, Option<Vec<String>>) {
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
