use std::sync::Arc;

use crate::shared::token::Token;

struct AstNode {
    parent: Box<Option<AstNode>>,
    children: Arc<Option<Vec<AstNode>>>,
    node_token: Token,
}

impl AstNode {
    fn new(
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
