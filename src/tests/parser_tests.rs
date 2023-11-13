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
    fn make_single_blank_node() {}

    #[test]
    fn make_single_node_with_comma() {}

    #[test]
    fn parent_child() {}
}
