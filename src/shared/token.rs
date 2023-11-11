/// This is the main token library for the kotlin language.
///
/// The lexer will convert raw code into these for use with the parser in building trees.
///
/// For additions, the following format is used. This keeps things as ambiguous as possible.
///
/// // [Symbol]
/// [Symbol Name]
///
/// Example
/// ```rust
///    
///    pub enum Token {
///
///    // ( <- Symbol
///    LeftBracket // <- Symbol Name
///    }
/// ```
///

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
    LParen,

    // )
    RParen,

    // {
    LCurlyBrace,

    // }
    RCurlyBrace,

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

    // see above enum Ident
    Identifier(Ident),

    // \n
    Newline,

    // :
    Colon,
}
