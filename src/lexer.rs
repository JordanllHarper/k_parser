use std::fmt::Display;

// Tokens in the basic hello world application ()
#[derive(Debug, PartialEq, Clone)]
enum Ident {
    Fun,
    Println,
    Val,
    Var,
    NonIdentifiable(String),
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    // Anything that isn't a designated token
    StringSymbol { symbol: String }, //e.g.  "[Hello],[World]"
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
    // !
    Bang,
    // !=
    DoesNotEqual,
    // val, var, fun, println, etc...
    Identifier(Ident),
}

#[derive(Debug, Clone)]
struct Lexer {
    position: usize,
    input: String,
    character: Option<char>,
}

fn is_identifier_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn match_identifier(input: &str) -> Option<Ident> {
    match input {
        "fun" => Some(Ident::Fun),
        "println" => Some(Ident::Println),
        "val" => Some(Ident::Val),
        "var" => Some(Ident::Var),
        _ => None,
    }
}

/// iterates through a string looking for a non identifiable character
/// - usize = the length traversed in the operation
/// - Option<Ident> = whether an Ident was found
fn seek(input: &str) -> (usize, Ident) {
    let split_val_vec: Vec<&str> = input.split(|c| !is_identifier_character(c)).collect();

    let value = match split_val_vec.first() {
        Some(split) => split,
        None => return (1, Ident::NonIdentifiable(input.to_string())),
    };

    let amount_traversed = value.len();

    match match_identifier(value) {
        Some(i) => return (amount_traversed, i),
        None => return (amount_traversed, Ident::NonIdentifiable(value.to_string())),
    };
}

/// Reads a token given
fn read_identifier(lexer: &Lexer) -> (Lexer, Token) {
    let string_to_seek = &lexer.input.split_at(lexer.position).1;
    println!("String to seek = {string_to_seek}");
    let (amount_traversed, ident) = seek(&lexer.input.split_at(lexer.position).1);

    let new_lexer = lexer.advance(amount_traversed);

    let token = Token::Identifier(ident);

    (new_lexer, token)
}

impl Lexer {
    fn peek_for_operator(
        &self,
        value_to_check_for: char,
        default_value: Token,
        matched_value: Token,
    ) -> Token {
        let advance_char = self.advance(1).character;
        if let Some(next_char) = advance_char {
            if next_char == value_to_check_for {
                return matched_value;
            } else {
                return default_value;
            };
        };
        default_value
    }

    fn advance(&self, amount: usize) -> Lexer {
        let new_position = self.position + amount;
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
        let token = match self.character {
            Some(c) => {
                println!("{}", c);
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
                        let (lexer, token) = read_identifier(self);
                        return (lexer, Some(token));
                    }
                };

                Some(token)
            }

            None => None,
        };
        (self.advance(1), token)
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
    use std::vec;

    use crate::lexer::{self, Lexer};

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

    #[test]
    fn println_input_success() {
        let lexer = Lexer::new(String::from("println"));
        let expected = Token::Identifier(lexer::Ident::Println);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fn_input_success() {
        let lexer = Lexer::new(String::from("fun"));
        let expected = Token::Identifier(lexer::Ident::Fun);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn val_input_success() {
        let lexer = Lexer::new(String::from("val"));
        let expected = Token::Identifier(lexer::Ident::Val);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn var_input_success() {
        let lexer = Lexer::new(String::from("var"));
        let expected = Token::Identifier(lexer::Ident::Var);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fun_main_input_success() {
        let lexer = Lexer::new("fun main".to_string());

        //[fun] main
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(lexer::Ident::Fun), token.unwrap());

        //fun[]main
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());

        //fun [main]
        let (new_lexer, token) = new_lexer.next_token();

        assert_eq!(
            Token::Identifier(lexer::Ident::NonIdentifiable("main".to_string())),
            token.unwrap()
        );
    }

    #[test]
    fn fun_main_lp_rp_input_success() {
        let lexer = Lexer::new("fun main()".to_string());
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(lexer::Ident::Fun), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(lexer::Ident::NonIdentifiable("main".to_string())),
            token.unwrap()
        );

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LeftBracket, token.unwrap());

        let (_, token) = new_lexer.next_token();
        assert_eq!(Token::RightBracket, token.unwrap());
    }

    #[test]
    fn fun_main_lp_rp_block_input_success() {
        let lexer = Lexer::new("fun main(){}".to_string());
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(lexer::Ident::Fun), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(lexer::Ident::NonIdentifiable("main".to_string())),
            token.unwrap()
        );
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LeftBracket, token.unwrap());

        let (_, token) = new_lexer.next_token();
        assert_eq!(Token::RightBracket, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        println!("{:?}", token.clone().unwrap());
        assert_eq!(Token::LeftSquirly, token.unwrap());

        let (_, token) = new_lexer.next_token();
        assert_eq!(Token::RightSquirly, token.unwrap());
    }

    #[test]
    fn is_alphanumeric() {
        assert!('}'.is_alphanumeric())
    }
}
