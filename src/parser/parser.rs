use crate::shared::token::{Ident, Token};

// 2 * 2

// 2
// 2 *
// 2 * 2

// pub struct Node {
//     op: Token,
//     lhs: Box<Option<Token>>,
//     rhs: Box<Option<Token>>,
// }

///Maps a token to a node to be used in the tree
fn map_token_to_node(token: Token, lhs: Box<Option<Node>>, rhs: Box<Option<Node>>) -> Node {
    match token {
        Token::Identifier(i) => Node::Value(i),
        _ => Node::Operator { token, lhs, rhs },
    }
}

fn init_head(token: Token) -> Node {
    map_token_to_node(token, Box::new(None), Box::new(None))
}

#[derive(PartialEq)]
pub enum Node {
    // Operating parent of the children
    //
    // e.g.
    //     [*]
    //
    //  2      2
    //
    Operator {
        token: Token,
        lhs: Box<Option<Node>>,
        rhs: Box<Option<Node>>,
    },
    // final value of a tree e.g.   *
    //
    //                          [2]   [2]
    Value(Ident),
}

/// Parser to convert a vector of tokens into an abstract syntax tree
///
/// tokens - the tokens to parse
/// index - the index of this parser instance
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn advance(&self, amount: usize) -> Self {
        Self {
            tokens: self.tokens.clone(),
            index: self.index + 1,
        }
    }

    pub fn build_syntax_tree(&self) -> (Parser, Option<Node>) {
        self.parse_node(Box::new(None))
    }

    /// Builds an ast from the tokens
    /// Returns the most recent version of the tree and the current parser step
    ///
    /// self - the current parser state
    /// node - the current node being operated on
    fn parse_node(&self, node: Box<Option<Node>>) -> (Parser, Option<Node>) {
        let new_parser_state = self.advance(1);

        let token = match self.tokens.get(self.index) {
            Some(t) => t,
            None => return (new_parser_state, None),
        };

        // handle if the initial node given exists or not
        //
        todo!()
    }
}

fn fold_opt_node(token: Token, opt_node: Box<Option<Node>>) -> Node {
    if let Some(unwrapped_node) = opt_node {
        compare_node_to_token(token, unwrapped_node)
    } else {
        init_head(token)
    }
}

fn compare_node_to_token(token: Token, node: Node) -> Node {
    node
}
