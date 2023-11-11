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

    // when
    When,

    // if
    If,

    // else
    Else,

    // anything that doesn't match above
    NonIdentifiable(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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

    // ||
    Or,

    // &&
    And,

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

    // <
    LAngleBracket,

    // >
    RAngleBracket,
}
