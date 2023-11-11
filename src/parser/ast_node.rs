use std::sync::Arc;

use crate::shared::token::Token;

/// A node of the AST.
///
/// Contains a token, the parent and the child of this node.
///
/// The parent and children will be None if there is none.
pub struct AstNode {
    parent: Box<Option<AstNode>>,
    children: Arc<Option<Vec<AstNode>>>,
    node_token: Token,
}

impl AstNode {
    /// Create a new node of the AST
    pub fn new(
        parent: Box<Option<AstNode>>,
        children: Arc<Option<Vec<AstNode>>>,
        node_token: Token,
    ) -> Self {
        Self {
            parent,
            children,
            node_token,
        }
    }
}
