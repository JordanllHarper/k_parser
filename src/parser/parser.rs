use std::sync::Arc;

use crate::shared::token::{Ident, Token};

struct AstNode {
    parent: Box<Option<AstNode>>,
    children: Arc<Option<Vec<AstNode>>>,
}
