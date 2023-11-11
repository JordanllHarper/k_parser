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

struct AstParser {
    tokens: Vec<Token>,
    current_token_position: u16,
}
