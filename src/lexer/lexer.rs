use crate::shared::token::{Ident, Token};

#[derive(Debug, Clone)]
pub struct Lexer {
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
        "main" => Some(Ident::Main),
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

fn read_identifier(lexer: &Lexer) -> (Lexer, Token) {
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

    pub fn new(input: String) -> Lexer {
        let first_char = input.chars().nth(0);
        Lexer {
            input,
            position: 0,
            character: first_char,
        }
    }

    pub fn next_token(&self) -> (Lexer, Option<Token>) {
        let token = match self.character {
            Some(c) => {
                let token = match c {
                    '(' => Token::LeftBracket,
                    ')' => Token::RightBracket,
                    '{' => Token::LeftSquirly,
                    '}' => Token::RightSquirly,
                    '\"' => Token::Quote,
                    '\'' => Token::Quote,
                    '\n' => Token::Newline,
                    ',' => Token::Comma,
                    ' ' => Token::Space,
                    '+' => Token::Plus,
                    '.' => Token::Period,
                    '-' => Token::Minus,
                    '!' => self.peek_for_operator('=', Token::Bang, Token::DoesNotEqual),
                    '?' => self.peek_for_operator('.', Token::Question, Token::Safecall),
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
    pub fn collect(self) -> Vec<Token> {
        let mut vec_of_tokens = Vec::new();

        let mut lexer_state = self;

        loop {
            let (lexer, opt_token) = lexer_state.next_token();

            match opt_token {
                Some(token) => vec_of_tokens.push(token),
                None => break,
            }

            lexer_state = lexer;
        }

        vec_of_tokens
    }
}

// Shopkeeper - Customer
// Something <- Bread
// Nothing   <- Obscure mango
//
//
//
//
//
