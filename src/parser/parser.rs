use std::sync::Arc;

use crate::shared::token::Token;

use super::ast_node::{AstNode, NodeType, SemanticAstNode};

pub trait Parser {
    type Node;
    /// Gets an updated tree root with the result of this Node.
    ///
    /// Will return the same tree if no more tokens are remaining.
    fn update_tree(self) -> (Self::Node, Self);
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
        let start_token = tokens.get(0).to_owned();

        let start_node = match start_token.cloned() {
            Some(token) => AstNode::new(NodeType::Parent {
                semantics: ParentSemantics::Root,
                children: Arc::new(vec![AstNode::new(NodeType::Child(token))]),
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

    fn advance(self) -> AstParser {
        let new_position = self.current_token_position + 1;
        let tokens = self.tokens;
        let tokens_binding = tokens.clone();
        let new_token = tokens_binding.get(new_position).cloned();
        AstParser {
            tokens,
            current_token_position: new_position,
            current_token: new_token,
            current_tree_root: self.current_tree_root.clone(),
        }
    }
}

impl Parser for AstParser {
    fn update_tree(self) -> (AstNode, AstParser) {
        let previous_root = &self.current_tree_root.to_owned();

        // TODO: Update previously added children with
        // new node status if not already done so
        // TODO: add new node to copied tree

        let new_parser = self.advance();
        (previous_root.clone(), new_parser)
    }
}
