#[cfg(test)]
mod parser_tests {
    use crate::{
        parser::parser::{Node, Parser},
        shared::token::{Ident, Token},
    };

    #[test]
    fn two_m_two_success() {
        let data = vec![Token::Identifier(Ident::NonIdentifiable("2".to_string()))];

        let expected = Node::Value(Ident::NonIdentifiable("2".to_string()));

        let parser = Parser::new(data);

        //TODO: Implement
        let actual = parser.build_syntax_tree();

        assert_eq!(expected, actual);
    }
}
