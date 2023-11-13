use std::sync::Arc;

use crate::shared::token::Token;

/// A node of the AST.
///
/// Contains a token, the parent and the child of this node.
///
/// The parent and children will be None if there is none.
#[derive(Clone, Debug)]
pub struct AstNode {
    node_type: NodeType,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Child(Token), // ends of the tree
    Parent {
        semantics: ParentSemantics,
        children: Arc<Vec<AstNode>>,
    }, // parents that join the end
}

#[derive(Clone, Debug)]
pub enum ParentSemantics {
    Function, // [fun f () { ... }]
    Identifier,
    ArgumentList, // fun f [(args...)] { ... }
    Variable,     // var x = 3
    Value,        // val y = 42
    Root,
}

impl AstNode {
    pub fn new(node_type: NodeType) -> Self {
        Self { node_type }
    }
}
