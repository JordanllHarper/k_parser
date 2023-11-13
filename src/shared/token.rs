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
pub enum Keyword {
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

    // |
    Pipe,

    // &
    Ampersand,

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

    // see above enum Keyword
    Keyword(Keyword),

    // Other value - anything not part the language constructs.
    NonIdentifiable(String),
    // \n
    Newline,

    // :
    Colon,

    // <
    LAngleBracket,

    // >
    RAngleBracket,
}
