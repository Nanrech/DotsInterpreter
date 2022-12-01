use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let file: String = read_to_string("src/main.dots")
        .expect("Couldn't read file.");

    let tokens: Vec<&str> = file
        .trim()
        .split(";\n")
        .collect();

    let dots_eop: &str = ".;";
    let dots_print: &str = "..";
    let dots_var: &str = "...";

    let mut dots_variables = HashMap::new();

    assert!(tokens.ends_with(&[dots_eop]));

    for token in tokens {
        if token == dots_eop {
            println!("y");
            return;
        }
        else if token.starts_with(dots_var) {
            // Tokenize: [...][name][value]
            // Everything else can be ignored so
            // we just don't look at whatever is after that
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
            let split_token: Vec<&str> = token
                .split(" ")
                .collect();
            for c in split_token {
                println!("{}", c);
            }
        }
    }
    // println!("{}", file);

}
