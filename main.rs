use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {
    let file: String = read_to_string("src/main.dots")
        .expect("Couldn't read file.");

    let tokens: Vec<&str> = file
        .trim()
        .split(";\n")
        .collect();

    // Bit ugy but idk
    let dots_eop: &str = ".;";
    let dots_print: &str = "..";
    let dots_var: &str = "...";

    // All vars are global
    let mut dots_variables = HashMap::new();

    assert!(tokens[tokens.len() - 1].ends_with(dots_eop));

    for t in tokens {
        // Just dupe it so we can remove the leading dots
        let mut token = String::from(t);

        println!("{}", token);

        if token == dots_eop {
            println!("Reached the end of the program.");
            return;
        }
        else if token.starts_with(dots_var) {
            // Tokenize: [...][name][value]
            // Everything else can be ignored so
            // we just don't look at whatever is after that

            for _ in 0..=2 {
                token.remove(0);
            }

            println!("{}", token);

            let split_token: Vec<&str> = token
                .split(" ")
                .collect();

            // TODO: Add some logic to accept `... name` & `...name value` as valid
            // TODO: maybe throw an error if the syntax is invalid?

            dots_variables.insert(split_token[1].to_string(),
                                  split_token[2].to_string());

        }
        else if token.starts_with(dots_print) {
            // TODO: Slice token starting from ..!
            // Imma go with .name. for printing variables because why not

            for _ in 0..=1 {
                token.remove(0);
            }

            let split_token: Vec<&str> = token
                .split(" ")
                .collect();
        }
    }
}
