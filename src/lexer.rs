use std::collections::HashMap;

// Tokens in the basic hello world application ()
#[derive(Debug, PartialEq, Clone)]
enum Token {
    // Anything that isn't a designated token
    StringSymbol { symbol: String }, //e.g. fun [main]() or "[Hello],[World]"
    // ()
    LeftBracket,
    RightBracket,
    // {}
    LeftSquirly,
    RightSquirly,
    // println
    // " || '
    Quote,
    // ,
    Comma,
    Space,
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
struct Lexer {
    position: usize,
    input: String,
    character: Option<char>,
}
impl Lexer {
    fn advance(&self) -> Lexer {
        Lexer {
            position: self.position + 1,
            input: self.input.clone(),
            character: self.character,
        }
    }

    fn new(input: String) -> Lexer {
        let character = input.chars().next();
        Lexer {
            input,
            position: 0,
            character,
        }
    }

    fn next_token(&self) -> (Lexer, Option<Token>) {
        let character = self.input.chars().nth(self.position);
        let token = match character {
            Some(c) => {
                let token = match c {
                    '(' => Token::LeftBracket,
                    ')' => Token::RightBracket,
                    '{' => Token::LeftSquirly,
                    '}' => Token::RightSquirly,
                    '\"' => Token::Quote,
                    '\'' => Token::Quote,
                    ',' => Token::Comma,
                    ' ' => Token::Space,
                    '+' => Token::Plus,
                    '=' => Token::Minus,
                    _ => Token::StringSymbol {
                        symbol: String::from("todo"),
                    },
                };

                Some(token)
            }

            None => None,
        };

        (self.advance(), token)
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, ()> {
    //fun main() { } -> [fun],[main()],[{],[}]

    let result = Lexer::new(input.to_string()).next_token();
    let lexer = result.0;
    let token = result.1;

    Ok(vec![])
}

// problem
// we currently assess everything on the character level - but keywords are multi character

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::{tokenize, Token};

    #[test]
    fn space_input_success() {
        let lexer = Lexer::new(String::from(" "));
        let expected = Token::Space;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lb_input_success() {
        let lexer = Lexer::new(String::from("("));
        let expected = Token::LeftBracket;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn rb_input_success() {
        let lexer = Lexer::new(String::from(")"));
        let expected = Token::RightBracket;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn ls_input_success() {
        let lexer = Lexer::new(String::from("{"));
        let expected = Token::LeftSquirly;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn rs_input_success() {
        let lexer = Lexer::new(String::from("}"));
        let expected = Token::RightSquirly;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn comma_input_success() {
        let lexer = Lexer::new(String::from(","));
        let expected = Token::Comma;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn quote_input_success() {
        let lexer = Lexer::new(String::from("\'"));
        let expected = Token::Quote;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn double_quote_input_success() {
        let lexer = Lexer::new(String::from("\""));
        let expected = Token::Quote;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
}
