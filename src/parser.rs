pub mod parser {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Token {
        pub body: String,
    }

    impl Token {
        pub fn new(body: String) -> Token {
            Token { body }
        }
    }

    pub fn parse_into_tokens(line: String, delimiters: &HashMap<String, String>) -> Vec<Token> {
        let mut vector_of_tokens: Vec<Token> = Vec::new();
        let mut current_body = String::new();

        for character in line.chars() {
            current_body = match delimiters.get(&character.to_string()) {
                Some(_) => {
                    if current_body.len() != 0 {
                        vector_of_tokens.push(Token::new(current_body.to_string()));
                    }
                    vector_of_tokens.push(Token::new(character.to_string()));
                    current_body = String::new();
                    continue;
                }

                None => current_body,
            };

            current_body.push(character);

            current_body = match delimiters.get(&current_body) {
                None => current_body,

                Some(_) => {
                    vector_of_tokens.push(Token::new(current_body));
                    String::new()
                }
            };
        }

        return vector_of_tokens;
    }
}
