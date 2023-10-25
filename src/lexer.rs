use std::collections::HashMap;

// Tokens in the basic hello world application ()
#[derive(Debug, PartialEq, Clone)]
enum Token {
    //fun
    Fun, //e.g. [fun] main()
    //Anything that isn't a designated token
    StringSymbol { symbol: String }, //e.g. fun [main]() or "[Hello],[World]"
    // ()
    LeftBracket,
    RightBracket,
    // {}
    LeftSquirly,
    RightSquirly,
    // println
    Println,
    // " || '
    Quote,
    // ,
    Comma,
}

fn tokenize(input: &str) -> Result<Vec<Token>, ()> {
    let map: HashMap<String, Token> = HashMap::from([
        (String::from("fun"), Token::Fun),
        (String::from("("), Token::LeftBracket),
        (String::from(")"), Token::RightBracket),
        (String::from("{"), Token::LeftSquirly),
        (String::from("}"), Token::RightSquirly),
        (String::from("println"), Token::Println),
        (String::from("\""), Token::Quote),
        (String::from(","), Token::Comma),
    ]);
    //Remove spaces
    let tokens: Vec<Token> = input
        .split(' ')
        .map(|v| {
            let token = &map[v];
            token.clone()
        })
        .collect::<Vec<Token>>();
    //fun main() { } -> [fun],[main()],[{],[}]
    //
    //
    //
    //

    Ok(tokens)
}

// fun[ ]main - space delimited
// ([]) - character delimited
// Which means we need to handle keywords on the space delimit level + characters on chars level

#[cfg(test)]
mod tests {

    use super::{tokenize, Token};

    #[test]
    fn it_passes() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn tokenize_with_fun_keyword() {
        let expected = vec![Token::Fun];

        let actual = tokenize("fun").unwrap();

        assert_eq!(expected[0], actual[0]);
    }
    #[test]
    fn tokenize_with_lb_symbol() {
        let expected = vec![Token::LeftBracket];

        let actual = tokenize("(").unwrap();

        assert_eq!(expected[0], actual[0]);
    }
    #[test]
    fn tokenize_with_rb_symbol() {
        let expected = vec![Token::RightBracket];

        let actual = tokenize(")").unwrap();

        assert_eq!(expected[0], actual[0]);
    }
    #[test]
    fn tokenize_with_ls_symbol() {
        let expected = vec![Token::LeftSquirly];

        let actual = tokenize("{").unwrap();

        assert_eq!(expected[0], actual[0]);
    }
    #[test]
    fn tokenize_with_rs_symbol() {
        let expected = vec![Token::RightSquirly];

        let actual = tokenize("}").unwrap();

        assert_eq!(expected[0], actual[0]);
    }

    #[test]
    fn tokenize_with_println_symbol() {
        let expected = vec![Token::Println];

        let actual = tokenize("println").unwrap();

        assert_eq!(expected[0], actual[0]);
    }
    #[test]
    fn tokenize_with_quote_symbol() {
        let expected = vec![Token::Quote];

        let actual = tokenize("\"").unwrap();

        assert_eq!(expected[0], actual[0]);
    }
    #[test]
    fn tokenize_with_comma_symbol() {
        let expected = vec![Token::Comma];

        let actual = tokenize(",").unwrap();

        assert_eq!(expected[0], actual[0]);
    }
}
