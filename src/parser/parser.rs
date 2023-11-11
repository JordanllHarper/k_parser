use crate::shared::token::Token;

use super::ast_node::AstNode;

trait Parser {
    /// Gets an updated tree root with the result of this Node.
    ///
    /// Will return the same tree if no more tokens are remaining.
    fn update_tree(&self) -> (AstNode, Self);
}

/// The implementation struct for the parser.
///
/// Contains the tokens of the building operation, the current position of the parser
/// and the current token being operated on.
struct AstParser {
    tokens: Vec<Token>,
    current_token_position: usize,
    current_token: Option<Token>,
}

impl AstParser {
    fn new(tokens: Vec<Token>) -> Self {
        let start_token = tokens.get(0).cloned();
        Self {
            tokens,
            current_token_position: 0,
            current_token: start_token,
        }
    }

    fn advance(&self) -> Self {
        let new_position = self.current_token_position + 1;
        let new_token = self.tokens.get(new_position);
        Self {
            tokens: self.tokens.clone(),
            current_token_position: new_position,
            current_token: new_token.cloned(),
        }
    }
}

impl Parser for AstParser {
    fn update_tree(&self) -> (AstNode, Self) {
        todo!()
    }
}
