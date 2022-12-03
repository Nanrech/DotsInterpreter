use std::fs::read_to_string;
use std::collections::HashMap;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let file: String = read_to_string("src/main.dots")
        .expect("Fatal Error: Couldn't read file.");

    let binding = file
        .replace('\n', "");

    let tokens: Vec<&str> = binding
        .trim()
        .split(";")
        .collect();

    // A bit ugy
    const DOTS_EOP: &str = ".";
    const DOTS_PRINT: &str = "..";
    const DOTS_VAR: &str = "...";

    // All vars are global
    let mut dots_variables: HashMap<String, String> = HashMap::new();

    // There should probably be a way to check if the program ends with a .;

    for t in tokens {
        let mut token: String = String::from(t);

        if token == DOTS_EOP {
            println!("\nReached the end of the program.");
            return;
        }

        else if token.starts_with(DOTS_VAR) {
            // Remove the ... from the token
            for _ in 0..3 {
                // Remove the ...
                token.remove(0);
            }
            let token: String = String::from(token
                .trim()
            );

            let split_token: Vec<&str> = token
                .split(" ")
                .collect();

            if split_token.len() > 2 {
                println!("Syntax error: Expected `...name value`, got '{}'.", token);
                return;
            }

            // Considering there's nothing to do with variables,
            // overwrite time
            dots_variables.insert(
                split_token[0]
                    // Needs to be parsed into a String because otherwise
                    // it gets dropped
                    .parse()
                    .unwrap(),
                split_token[1]
                    .parse()
                    .unwrap()
            );
        }
        else if token.starts_with(DOTS_PRINT) {
            /*
            Since there's nothing to do with variables I'm just not going to
            try to implement printing them, raw printing time it is, bay bee.
            */

            // Remove the .. from the token
            for _ in 0..2 {
                token.remove(0);
            }

            // Yeag
            print!("{}", token);
        }
    }
}
