use crate::shared::token::Ident;

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
        "when" => Some(Ident::When),
        "if" => Some(Ident::If),
        "else" => Some(Ident::Else),
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
