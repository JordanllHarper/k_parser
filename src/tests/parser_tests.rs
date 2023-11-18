#[cfg(test)]
pub mod parser_tests {
    use std::sync::Arc;

    use crate::{
        parser::{
            ast_node::{AstNode, NodeType, ParentSemantics},
            parser::{AstParser, Parser},
        },
        shared::token::*,
    };

    #[test]
    fn one_token_node() {
        let test_data = vec![Token::Keyword(Keyword::Fun)];
        let parser = AstParser::new(test_data);
        let expected = AstNode::new(NodeType::Parent {
            semantics: ParentSemantics::Root,
            children: Arc::new(vec![AstNode::new(NodeType::Child(Token::Keyword(
                Keyword::Fun,
            )))]),
        });
        let (node, _parser) = parser.update_tree();
        assert_eq!(expected, node);
    }
}
