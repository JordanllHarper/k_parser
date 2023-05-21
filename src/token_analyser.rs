pub mod token_analyser {
    use std::collections::HashMap;

    use crate::{operator::operator::Operator, parser::parser::Token};
    pub struct Node {
        value: NodeObj,
        lhs: Box<Option<Node>>,
        rhs: Box<Option<Node>>,
    }
    impl Node {
        pub fn new(value: NodeObj, lhs: Box<Option<Node>>, rhs: Box<Option<Node>>) -> Node {
            Node { value, lhs, rhs }
        }
    }

    pub enum NodeObj {
        Operator(Operator),
        Value(Value),
        Head,
    }

    pub enum Value {
        StringValue(String),
    }

    //pub fn analyse(tokens: Vec<Token>) -> Node {
    //  let map = build_map();
    //   let head = Node::new(Head, Box::new(None), Box::new(None));
    //}

    //pub fn parse_body(operator: Operator, remaining_tokens: Vec<Token>) -> Node {
    //get the body from the remaining tokens

    //will be the operator node which contains the other nodes

    //        Node::new(value, lhs, rhs)
    //}

    //
    //println("Hello ,World!")
    //
    //
    //

    pub fn build_map() -> HashMap<String, Operator> {
        let mut map = HashMap::new();

        map.insert(String::from("println"), Operator::Print);
        map
    }
}
