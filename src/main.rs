mod lexer;
use std::collections::HashMap;

fn main() {
    let mut delimiters = HashMap::new();
    delimiters.insert(String::from("println"), String::from("println"));
    delimiters.insert(String::from("("), String::from("("));
    delimiters.insert(String::from(")"), String::from(")"));
    delimiters.insert(String::from("\""), String::from("\""));
    let line = String::from("println(\"Hello, World!\")");
}
