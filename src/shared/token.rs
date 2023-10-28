#[derive(Debug, PartialEq, Clone)]
pub enum Ident {
    Fun,
    Println,
    Val,
    Var,
    NonIdentifiable(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Anything that isn't a designated token
    StringSymbol { symbol: String }, //e.g.  "[Hello],[World]"
    // ()
    LeftBracket,
    RightBracket,
    // {}
    LeftSquirly,
    RightSquirly,
    // " || '
    Quote,
    // ,
    //
    Comma,
    // .
    Period,
    // [ ]
    Space,
    // +
    Plus,
    // -
    Minus,
    // ==
    Assign,
    // =
    Equals,
    // !
    Bang,
    // !=
    DoesNotEqual,
    // val, var, fun, println, etc...
    Identifier(Ident),
    // \n
    Newline,
}
