pub trait Parser {
    // the node type of the parser
    type Node;
    /// Gets an updated tree root with the result of this Node.
    ///
    /// Will return the same tree if no more tokens are remaining.
    fn update_tree(self) -> (Self::Node, Self);
}
