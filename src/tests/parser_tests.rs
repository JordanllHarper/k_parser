#[cfg(test)]
mod parser_tests {
    // TODO: To test:
    // - [ ] Make a single blank node ast
    // - [ ] Make a single node ast
    // - [ ] Make with 2 children
    // - [ ] Make with 3 children
    use std::sync::Arc;

    use crate::{
        parser::{
            ast_node::AstNode,
            parser::{AstParser, Parser},
        },
        shared::token::Token,
    };

    #[test]
    fn make_single_blank_node() {
        let tokens: Vec<Token> = vec![];
        let parser = AstParser::new(tokens);
        let expected = AstNode::new(
            Arc::new(None),
            Token::Identifier(crate::shared::token::Ident::NonIdentifiable(String::from(
                "",
            ))),
        );
        let (node, _parser) = parser.update_tree();

        assert_eq!(expected.node_token, node.node_token)
    }

    #[test]
    fn make_single_node_with_comma() {
        let tokens: Vec<Token> = vec![Token::Comma];
        let parser = AstParser::new(tokens);
        let expected = AstNode::new(Arc::new(None), Token::Comma);
        let (node, _parser) = parser.update_tree();

        assert_eq!(expected.node_token, node.node_token)
    }

    #[test]
    fn parent_child() {
        let tokens: Vec<Token> = vec![Token::Comma, Token::Space];
        let expected_children = vec![AstNode::new(Arc::new(None), Token::Space)];
        let parser = AstParser::new(tokens);
        let expected = AstNode::new(Arc::new(Some(expected_children)), Token::Comma);
        let (_, parser) = parser.update_tree();
        let (first_node, _parser) = parser.update_tree(); //tree with 1 child
        assert_eq!(expected.node_token, first_node.node_token);
        let expected_child = expected
            .children
            .as_ref()
            .clone()
            .unwrap()
            .get(0)
            .unwrap()
            .node_token
            .clone();
        let actual_child = first_node
            .children
            .as_ref()
            .clone()
            .unwrap()
            .get(0)
            .unwrap()
            .node_token
            .clone();
        assert_eq!(expected_child, actual_child)
        // let second_node = first_node.children.unwrap();
    }
}
