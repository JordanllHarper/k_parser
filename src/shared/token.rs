#[derive(Debug, PartialEq, Clone)]
pub enum Ident {
    Fun,
    Main,
    Println,
    Val,
    Var,
    NonIdentifiable(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Anything that isn't a designated token
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
    // *
    Multiply,
    // /
    Divide,
    // ==
    Assign,
    // =
    Equals,
    // !
    Bang,
    // ?
    Question,
    //?.whatever
    Safecall,
    // !=
    DoesNotEqual,
    // val, var, fun, println, etc...
    Identifier(Ident),
    // \n
    Newline,
}
