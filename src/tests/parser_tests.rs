#[cfg(test)]
pub mod parser_tests {
    use std::sync::Arc;

    use crate::{
        parser::{
            ast_node::{AstNode, NodeType},
            non_semantic::parser::AstNodeParser,
            parser::Parser,
        },
        shared::token::*,
    };

    #[test]
    fn init_initial_pass() {
        let test_data = vec![Token::Keyword(Keyword::Fun)];
        let parser = AstNodeParser::new(test_data);
        let expected = AstNode::new(NodeType::Parent {
            children: Arc::new(vec![AstNode::new(NodeType::Child(Token::Keyword(
                Keyword::Fun,
            )))]),
        });
        let (node, _parser) = parser.update_tree();
        assert_eq!(expected, node);
    }
}
