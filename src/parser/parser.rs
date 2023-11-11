use crate::shared::token::Token;

use super::ast_node::AstNode;

trait Parser {
    /// Advances the parser state to the next token.
    fn advance(&self) -> Self;

    /// Gets an updated tree root with the result of this Node.
    ///
    /// Will return the same tree if no more tokens are remaining.
    fn next_node(&self) -> (AstNode, Self);
}

/// The implementation struct for the parser.
///
/// Contains the tokens of the building operation and the current position of the parser.
struct AstParser {
    tokens: Vec<Token>,
    current_token_position: u16,
}

impl AstParser {
    fn new(tokens: Vec<Token>, current_token_position: u16) -> Self {
        Self {
            tokens,
            current_token_position,
        }
    }
}

impl Parser for AstParser {
    fn advance(&self) -> Self {
        todo!()
    }

    fn next_node(&self) -> (AstNode, Self) {
        todo!()
    }
}
