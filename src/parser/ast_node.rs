use std::sync::Arc;

use crate::shared::token::Token;

/// A node of the AST.
///
/// parent - the parent of this node
/// children - the children of this node
/// node_token - the token of this node  
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
