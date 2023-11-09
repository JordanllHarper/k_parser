/// This is the main token library for the kotlin language.
///
/// The lexer will convert raw code into these for use with the parser in building trees.
///
/// For additions - format is as follows
///
/// //[symbol]
/// [SymbolName]
///
/// E.g.
/// ```rust
///    
///    // (
///    LeftBracket
///
/// ```

#[derive(Debug, PartialEq, Clone)]
pub enum Ident {
    // fun
    Fun,
    // main
    Main,
    // val
    Val,
    // var
    Var,
    // anything that doesn't match above
    NonIdentifiable(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Anything that isn't a designated token
    // (
    LeftBracket,

    // )
    RightBracket,

    // {
    LeftSquirly,

    // }
    RightSquirly,

    // " or '
    Quote,

    // ,
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

    // see above
    Identifier(Ident),

    // \n
    Newline,
}
