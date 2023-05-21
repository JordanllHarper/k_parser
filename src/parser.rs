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
    pub fn filter_string(tokens: Vec<Token>) -> Vec<Token> {
        let mut current_body: Vec<Token> = Vec::new();

        let mut record_token = false;
        for token in tokens {
            if token.body == "\"" && record_token {
                record_token = false;
                continue;
            }
            if token.body == "\"" {
                record_token = true;
                continue;
            }

            if record_token {
                current_body.push(token)
            }
        }
        current_body
    }

    pub fn filter_brackets(tokens: Vec<Token>) -> Vec<Token> {
        let mut current_body: Vec<Token> = Vec::new();

        let mut record_in_brackets = false;
        for token in tokens {
            if token.body == "(" {
                record_in_brackets = true;
                continue;
            }
            if token.body == ")" {
                record_in_brackets = false;
                continue;
            }

            if !record_in_brackets {
                continue;
            }

            current_body.push(token);
        }
        current_body
    }
}
