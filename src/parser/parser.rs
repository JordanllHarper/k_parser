use std::sync::Arc;

use crate::shared::token::{Ident, Token};

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
    current_tree_root: AstNode,
}

impl AstParser {
    fn new(tokens: Vec<Token>) -> Self {
        let start_token = tokens.get(0);

        let start_node = match start_token {
            Some(token) => AstNode::new(Box::new(None), Arc::new(None), token.clone()),
            None => AstNode::new(
                Box::new(None),
                Arc::new(None),
                Token::Identifier(Ident::NonIdentifiable(String::from(""))),
            ),
        };
        Self {
            tokens: tokens.clone(),
            current_token_position: 0,
            current_token: start_token.cloned(),
            current_tree_root: start_node,
        }
    }

    fn advance(&self, tree_root: AstNode) -> Self {
        let new_position = self.current_token_position + 1;
        let new_token = self.tokens.get(new_position);
        Self {
            tokens: self.tokens.clone(),
            current_token_position: new_position,
            current_token: new_token.cloned(),
            current_tree_root: tree_root,
        }
    }
}

impl Parser for AstParser {
    // bare minimum functionality
    // One node -> e.g. ,
    //
    // Process:
    // Read through tokens
    // If current token is None, return the root with no operations
    //

    fn update_tree(&self) -> (AstNode, Self) {
        todo!()
    }
}
