use crate::shared::token::{Keyword, Token};

use super::ast_node::{AstNode, NodeType, ParentSemantics};

pub trait Parser {
    /// Gets an updated tree root with the result of this Node.
    ///
    /// Will return the same tree if no more tokens are remaining.
    fn update_tree(&self) -> (AstNode, Self);
}

/// The implementation struct for the parser.
///
/// Contains the tokens of the building operation, the current position of the parser
/// and the current token being operated on.
pub struct AstParser {
    tokens: Vec<Token>,
    current_token_position: usize,
    current_token: Option<Token>,
    current_tree_root: AstNode,
}

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let start_token = tokens.get(0);

        let start_node = match start_token {
            Some(token) => AstNode::new(NodeType::Parent {
                semantics: ParentSemantics::Root,
                children: vec![].into(),
            }),
            None => AstNode::new(NodeType::Child(Token::NonIdentifiable("".to_string()))),
        };
        Self {
            tokens: tokens.clone(),
            current_token_position: 0,
            current_token: start_token.cloned(),
            current_tree_root: start_node,
        }
    }

    fn advance(&self) -> Self {
        let new_position = self.current_token_position + 1;
        let new_token = self.tokens.get(new_position);
        Self {
            tokens: self.tokens.clone(),
            current_token_position: new_position,
            current_token: new_token.cloned(),
            current_tree_root: self.current_tree_root.clone(),
        }
    }
}

impl Parser for AstParser {
    fn update_tree(&self) -> (AstNode, Self) {
        let new_parser = self.advance();
        (new_parser.current_tree_root.clone(), new_parser)
    }
}
