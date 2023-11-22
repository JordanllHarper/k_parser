use std::sync::Arc;

use crate::shared::token::Token;

/// A node of the AST.
///
/// This only lays out the tokens in a structural order. Semantic processing is done after this
/// initial tree is built.
///
/// Contains a token, the parent and the child of this node.
///
/// The parent and children will be None if there is none.
#[derive(Clone, Debug, PartialEq)]
pub struct AstNode {
    node_type: NodeType,
}

impl AstNode {
    pub fn new(node_type: NodeType) -> Self {
        Self { node_type }
    }
}

/// Non semanatic Node type of the non semantic tree.
///
/// Difference between this and [SemanticNodeType] is this lacks the [ParentSemantics].
#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Child(Token),                           // ends of the tree
    Parent { children: Arc<Vec<AstNode>> }, // parents that join the end
}
    Child(Token), // ends of the tree
    Parent {
        semantics: ParentSemantics,
        children: Arc<Vec<AstNode>>,
    }, // parents that join the end
}

#[derive(Clone, Debug, PartialEq)]
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
