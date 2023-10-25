use std::collections::HashMap;

// Tokens in the basic hello world application ()
#[derive(Debug, PartialEq, Clone)]
enum Token {
    // Anything that isn't a designated token
    StringSymbol { symbol: String }, //e.g. [fun] [main]() or "[Hello],[World]"
    // ()
    LeftBracket,
    RightBracket,
    // {}
    LeftSquirly,
    RightSquirly,
    // " || '
    Quote,
    // ,
    Comma,
    // [ ]
    Space,
    // +
    Plus,
    // -
    Minus,
    // ==
    Assign,
    // =
    Equals,
    Bang,
    DoesNotEqual,
}

#[derive(Debug, Clone)]
struct Lexer {
    position: usize,
    input: String,
    character: Option<char>,
}
impl Lexer {
    fn peek_for_operator(
        &self,
        value_to_check_for: char,
        default_value: Token,
        matched_value: Token,
    ) -> Token {
        let advance_char = self.advance().character;
        if let Some(next_char) = advance_char {
            if next_char == value_to_check_for {
                return matched_value;
            } else {
                return default_value;
            };
        };
        default_value
    }
    fn advance(&self) -> Lexer {
        let new_position = self.position + 1;
        let new_char = self.input.chars().nth(new_position);
        Lexer {
            position: new_position,
            input: self.input.clone(),
            character: new_char,
        }
    }

    fn new(input: String) -> Lexer {
        let first_char = input.chars().nth(0);
        Lexer {
            input,
            position: 0,
            character: first_char,
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
                    '-' => Token::Minus,
                    '!' => self.peek_for_operator('=', Token::Bang, Token::DoesNotEqual),
                    '=' => self.peek_for_operator('=', Token::Assign, Token::Equals),
                    _ => {
                        todo!()
                    }
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

    #[test]
    fn plus_input_success() {
        let lexer = Lexer::new(String::from("+"));
        let expected = Token::Plus;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn minus_input_success() {
        let lexer = Lexer::new(String::from("-"));
        let expected = Token::Minus;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn assign_input_success() {
        let lexer = Lexer::new(String::from("="));
        let expected = Token::Assign;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn equality_input_success() {
        let lexer = Lexer::new(String::from("=="));
        let expected = Token::Equals;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bang_input_success() {
        let lexer = Lexer::new(String::from("!"));
        let expected = Token::Bang;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn does_not_equal_input_success() {
        let lexer = Lexer::new(String::from("!="));
        let expected = Token::DoesNotEqual;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
}
