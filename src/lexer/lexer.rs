use crate::shared::token::{Ident, Token};

#[derive(Debug, Clone)]
/// The Lexer for converting a given input of type String to a stream of Tokens.
///
/// Uses the iterator pattern to provide immutability when moving from one token to the next.
///
/// Also comes with a collect method for String -> Vec<Token> convenience!
///
/// Mainly informed by TJ Devries's parser.
pub struct Lexer {
    position: usize,
    input: String,
    character: Option<char>,
}

/// Returns true if it is an identifier character.
/// E.g. a-zA-Z or _ -> characters that aren't non identifiable and specific to operations such as
/// + or -.
fn is_identifier_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// Matches an input of some identifier with it's token.
/// Some if it matches the list of identifiers.
/// None if it doesn't match.
fn match_identifier(input: &str) -> Option<Ident> {
    match input {
        "fun" => Some(Ident::Fun),
        "main" => Some(Ident::Main),
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

/// Reads an identifier such as a keyword 'fun' and tokenises
fn read_identifier(lexer: &Lexer) -> (Lexer, Token) {
    let (amount_traversed, ident) = seek(&lexer.input.split_at(lexer.position).1);

    let new_lexer = lexer.advance(amount_traversed);

    let token = Token::Identifier(ident);

    (new_lexer, token)
}

/// Lexer for creating tokens.
/// Immutable by design and inspired by TJ Devries's parser.
impl Lexer {
    /// Peeks for a second operator to tokenize double character operators --> e.g. "==" or "!="
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

    /// Advances the lexer a given amount. Returns a new lexer on the next character.
    fn advance(&self, amount: usize) -> Lexer {
        let new_position = self.position + amount;

        let new_char = self.input.chars().nth(new_position);
        Lexer {
            position: new_position,
            input: self.input.clone(),
            character: new_char,
        }
    }

    /// Creates a new lexer to traverse the given input.
    pub fn new(input: String) -> Lexer {
        let first_char = input.chars().nth(0);
        Lexer {
            input,
            position: 0,
            character: first_char,
        }
    }

    /// Gets the next token. See token.rs.
    pub fn next_token(&self) -> (Lexer, Option<Token>) {
        let token = match self.character {
            Some(c) => {
                let token = match c {
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    '{' => Token::LCurlyBrace,
                    '}' => Token::RCurlyBrace,
                    '\"' => Token::Quote,
                    '\'' => Token::Quote,
                    '\n' => Token::Newline,
                    ',' => Token::Comma,
                    ' ' => Token::Space,
                    '+' => Token::Plus,
                    '*' => Token::Multiply,
                    '-' => Token::Minus,
                    '/' => Token::Divide,
                    '.' => Token::Period,
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
    /// Convenience method for advancing lexer through the input and creating a vector of tokens.
    ///
    /// You probably want to use this the majority of the time.
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
