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
            Box::new(None),
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
        let expected = AstNode::new(Box::new(None), Arc::new(None), Token::Comma);
        let (node, _parser) = parser.update_tree();

        assert_eq!(expected.node_token, node.node_token)
    }
}
