use crate::shared::token::Token;

use super::lexer_utils::seek;
/// The Lexer for converting a given input of type String to a stream of Tokens.
///
/// Uses the iterator pattern to provide immutability when moving from one token to the next.
///
/// Also comes with a collect method for String -> Vec<Token> convenience!
///
/// Mainly informed by TJ Devries's parser.
#[derive(Debug, Clone)]
pub struct Lexer {
    position: usize,
    input: String,
    character: Option<char>,
}

/// Lexer for creating tokens.
/// Immutable by design and inspired by TJ Devries's parser.
impl Lexer {
    /// Peeks for a second operator to tokenize double character operators -> e.g. "==" or "!="
    ///
    /// value_to_check_for - Returns matched value if it finds this
    /// default_value - the default value if the check for value isn't found
    /// matched_value - the returned value if the value to check for isn't found
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

    /// Advances the lexer a given amount.
    ///
    /// amount - The amount to advance the lexer
    ///
    /// Returns a new lexer with the given position.
    fn advance(&self, amount: usize) -> Lexer {
        let new_position = self.position + amount;

        let new_char = self.input.chars().nth(new_position);
        Lexer {
            position: new_position,
            input: self.input.clone(),
            character: new_char,
        }
    }

    /// Reads an identifier such as a keyword 'fun', or a non identifiable such as 'hi', and tokenises
    fn read_keyword_or_non_identifiable(&self) -> (Lexer, Token) {
        let (amount_traversed, token) = seek(self.input.split_at(self.position).1);

        let new_lexer = self.advance(amount_traversed);

        (new_lexer, token)
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
                    ':' => Token::Colon,
                    '<' => Token::LAngleBracket,
                    '>' => Token::RAngleBracket,
                    '!' => self.peek_for_operator('=', Token::Bang, Token::DoesNotEqual),
                    '?' => self.peek_for_operator('.', Token::Question, Token::Safecall),
                    '=' => self.peek_for_operator('=', Token::Assign, Token::Equals),
                    '|' => self.peek_for_operator('|', Token::Pipe, Token::Or),
                    '&' => self.peek_for_operator('&', Token::Ampersand, Token::And),
                    _ => {
                        let (lexer, token) = self.read_keyword_or_non_identifiable();
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
