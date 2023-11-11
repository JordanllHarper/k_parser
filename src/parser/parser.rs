use crate::shared::token::Token;

struct Parser {
    tokens: Vec<Token>,
    current_token_position: u16,
}

