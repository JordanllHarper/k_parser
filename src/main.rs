use std::collections::HashMap;

use crate::parser::parser::parse_into_tokens;

mod parser;
mod tests;
mod token_analyser;

fn main() {
    let mut delimiters = HashMap::new();
    delimiters.insert(String::from("println"), String::from("println"));
    delimiters.insert(String::from("("), String::from("("));
    delimiters.insert(String::from(")"), String::from(")"));
    delimiters.insert(String::from("\""), String::from("\""));

    let line = String::from("println(\"Hello, World!\")");

    let tokens = parse_into_tokens(line, &delimiters);
    for token in tokens {
        println!("Token: {:?}", token);
    }
}
