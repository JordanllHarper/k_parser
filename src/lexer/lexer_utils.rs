use crate::shared::token::{Keyword, Token};

/// Returns true if it is an identifier character.
/// E.g. a-zA-Z or _ -> characters that aren't non identifiable and specific to operations such as
/// + or -.
pub fn is_identifier_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// Matches an input of some keyword with it's token.
/// Some if it matches the list of keyword.
/// None if it doesn't match.
pub fn match_keyword(input: &str) -> Option<Keyword> {
    match input {
        "fun" => Some(Keyword::Fun),
        "main" => Some(Keyword::Main),
        "val" => Some(Keyword::Val),
        "var" => Some(Keyword::Var),
        "when" => Some(Keyword::When),
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        _ => None,
    }
}

/// iterates through a string looking for a non identifiable character
/// - usize = the length traversed in the operation
/// - Option<Ident> = whether an Ident was found
pub fn seek(input: &str) -> (usize, Token) {
    let split_val_vec: Vec<&str> = input.split(|c| !is_identifier_character(c)).collect();

    let value = match split_val_vec.first() {
        Some(split) => split,
        None => return (1, Token::NonIdentifiable(input.to_string())),
    };

    let amount_traversed = value.len();

    match match_keyword(value) {
        Some(i) => return (amount_traversed, Token::Keyword(i)),
        None => return (amount_traversed, Token::NonIdentifiable(value.to_string())),
    };
}
