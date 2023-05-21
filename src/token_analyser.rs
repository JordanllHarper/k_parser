pub mod token_analyser {
    use std::collections::HashMap;

    pub struct Node {
        value: NodeObj,
        lhs: Box<Option<Node>>,
        rhs: Box<Option<Node>>,
    }

    pub enum NodeObj {
        Operator,
        Value,
    }

    pub enum Value {
        StringValue(String),
    }

    pub enum Operator {
        Print,
    }

    pub fn analyse(tokens: Vec<Token>) -> Node {
        for token in tokens {}
    }

    pub fn build_map() -> HashMap<String, Operator> {
        let map = HashMap::new();

        map.insert("println", Operator::Print);
        map
    }
}
