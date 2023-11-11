use crate::shared::token::Ident;

use super::lexer::Lexer;

/// Returns true if it is an identifier character.
/// E.g. a-zA-Z or _ -> characters that aren't non identifiable and specific to operations such as
/// + or -.
pub fn is_identifier_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// Matches an input of some identifier with it's token.
/// Some if it matches the list of identifiers.
/// None if it doesn't match.
pub fn match_identifier(input: &str) -> Option<Ident> {
    match input {
        "fun" => Some(Ident::Fun),
        "main" => Some(Ident::Main),
        "val" => Some(Ident::Val),
        "var" => Some(Ident::Var),
        _ => None,
    }
}

/// iterates through a string looking for a non identifiable character
/// - usize = the length traversed in the operation
/// - Option<Ident> = whether an Ident was found
pub fn seek(input: &str) -> (usize, Ident) {
    let split_val_vec: Vec<&str> = input.split(|c| !is_identifier_character(c)).collect();

    let value = match split_val_vec.first() {
        Some(split) => split,
        None => return (1, Ident::NonIdentifiable(input.to_string())),
    };

    let amount_traversed = value.len();

    match match_identifier(value) {
        Some(i) => return (amount_traversed, i),
        None => return (amount_traversed, Ident::NonIdentifiable(value.to_string())),
    };
}

/// Reads an identifier such as a keyword 'fun' and tokenises
pub fn read_identifier(lexer: &Lexer) -> (Lexer, Token) {
    let (amount_traversed, ident) = seek(&lexer.input.split_at(lexer.position).1);

    let new_lexer = lexer.advance(amount_traversed);

    let token = Token::Identifier(ident);

    (new_lexer, token)
}
