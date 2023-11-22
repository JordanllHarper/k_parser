use std::sync::Arc;

use crate::{
    parser::{
        ast_node::{AstNode, NodeType},
        parser::Parser,
    },
    shared::token::Token,
};

/// The implementation struct for the parser.
///
/// Contains the tokens of the building operation, the current position of the parser
/// and the current token being operated on.
pub struct AstNodeParser {
    tokens: Vec<Token>,
    current_token_position: usize,
    current_token: Option<Token>,
    current_tree_root: AstNode,
}

impl AstNodeParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let start_token = tokens.get(0).to_owned();

        let start_node = match start_token.cloned() {
            Some(token) => AstNode::new(NodeType::Parent {
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

    fn advance(self) -> AstNodeParser {
        let new_position = self.current_token_position + 1;
        let tokens = self.tokens;
        let tokens_binding = tokens.clone();
        let new_token = tokens_binding.get(new_position).cloned();
        AstNodeParser {
            tokens,
            current_token_position: new_position,
            current_token: new_token,
            current_tree_root: self.current_tree_root.clone(),
        }
    }
    /// Iterates through the tree and inserts the child
    /// Inserts new node at the end of the last child
    fn iterate_node(node: AstNode, node_to_add: Option<AstNode>) -> AstNode {
        todo!()
    }
}

impl Parser for AstNodeParser {
    type Node = AstNode;
    fn update_tree(self) -> (AstNode, AstNodeParser) {
        let previous_root = &self.current_tree_root.to_owned();

        // TODO: add new node to copied tree

        let new_parser = self.advance();
        (previous_root.clone(), new_parser)
    }
}
