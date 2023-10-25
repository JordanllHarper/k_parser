use std::collections::HashMap;

// Tokens in the basic hello world application ()
#[derive(Debug, PartialEq, Clone)]
enum Token {
    // Anything that isn't a designated token
    StringSymbol { symbol: String }, //e.g. fun [main]() or "[Hello],[World]"
    // ()
    LeftBracket,
    RightBracket,
    // {}
    LeftSquirly,
    RightSquirly,
    // println
    // " || '
    Quote,
    // ,
    Comma,
    Space,
}

#[derive(Debug, Clone)]
struct Lexer {
    position: usize,
    input: String,
    character: Option<char>,
}
fn map_character(character: Option<&char>) -> Option<Token> {
    match character {
        Some(c) => {
            let character_map: HashMap<char, Token> = HashMap::from([
                ('(', Token::LeftBracket),
                (')', Token::RightBracket),
                ('{', Token::LeftSquirly),
                ('}', Token::RightSquirly),
                ('\"', Token::Quote),
                ('\'', Token::Quote),
                (',', Token::Comma),
                (' ', Token::Space),
            ]);

            character_map.get(c).cloned()
        }
        None => None,
    }
}
// fn map_keyword(keyword: &String) -> Option<Token> {
//     let keyword_map: HashMap<String, Token> = HashMap::from([
//         (String::from("fun"), Token::Fun),
//         (String::from("println"), Token::Println),
//     ]);
//     keyword_map.get(keyword).cloned()
// }
impl Lexer {
    fn new(input: String) -> Lexer {
        let character = input.chars().next();
        let token = map_character(character.as_ref());
        Lexer {
            input,
            position: 0,
            character,
        }
    }

    fn next_token(&self) -> (Lexer, Option<Token>) {
        let character = self.input.chars().nth(self.position);
        let token = map_character(character.as_ref());

        (
            Lexer {
                position: self.position + 1,
                input: self.input.to_string(),
                character,
            },
            token,
        )
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, ()> {
    //fun main() { } -> [fun],[main()],[{],[}]

    let result = Lexer::new(input.to_string()).next_token();
    let lexer = result.0;
    let token = result.1;

    Ok(vec![])
}

// problem
// we currently assess everything on the character level - but keywords are multi character

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::{tokenize, Token};

    #[test]
    fn space_input_success() {
        let lexer = Lexer::new(String::from(" "));
        let expected = Token::Space;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lb_input_success() {
        let lexer = Lexer::new(String::from("("));
        let expected = Token::LeftBracket;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn rb_input_success() {
        let lexer = Lexer::new(String::from(")"));
        let expected = Token::RightBracket;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn ls_input_success() {
        let lexer = Lexer::new(String::from("{"));
        let expected = Token::LeftSquirly;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn rs_input_success() {
        let lexer = Lexer::new(String::from("}"));
        let expected = Token::RightSquirly;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn comma_input_success() {
        let lexer = Lexer::new(String::from(","));
        let expected = Token::Comma;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn quote_input_success() {
        let lexer = Lexer::new(String::from("\'"));
        let expected = Token::Quote;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn double_quote_input_success() {
        let lexer = Lexer::new(String::from("\""));
        let expected = Token::Quote;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
}
