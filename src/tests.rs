#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::parser::{filter_brackets, filter_string, parse_into_tokens, Token};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn parser_hello_world_test() {
        let data = "println(\"Hello, World!\")";

        let mut delimiters = HashMap::new();
        delimiters.insert(String::from("println"), String::from("println"));
        delimiters.insert(String::from("("), String::from("("));
        delimiters.insert(String::from(")"), String::from(")"));
        delimiters.insert(String::from("\""), String::from("\""));

        let mut expected: Vec<Token> = Vec::new();

        expected.push(Token::new(String::from("println")));
        expected.push(Token::new(String::from("(")));
        expected.push(Token::new(String::from("\"")));
        expected.push(Token::new(String::from("Hello, World!")));
        expected.push(Token::new(String::from("\"")));
        expected.push(Token::new(String::from(")")));

        let actual = parse_into_tokens(data.to_string(), &delimiters);
        for token in &actual {
            println!("{}", token.body);
        }

        assert_eq!(expected[0].body, actual[0].body);
        assert_eq!(expected[1].body, actual[1].body);
        assert_eq!(expected[2].body, actual[2].body);
        assert_eq!(expected[3].body, actual[3].body);
        assert_eq!(expected[4].body, actual[4].body);
        assert_eq!(expected[5].body, actual[5].body);
    }

    #[test]
    fn parser_filter_brackets_test() {
        let data = vec![
            Token::new(String::from("(")),
            Token::new(String::from("\"")),
            Token::new(String::from("Hello, World!")),
            Token::new(String::from("\"")),
            Token::new(String::from(")")),
        ];

        let expected = vec![
            Token::new(String::from("\"")),
            Token::new(String::from("Hello, World!")),
            Token::new(String::from("\"")),
        ];

        let actual = filter_brackets(data);

        assert_eq!(expected[0].body, actual[0].body);
        assert_eq!(expected[1].body, actual[1].body);
        assert_eq!(expected[2].body, actual[2].body);
    }
    #[test]
    fn parser_filter_string_test() {
        let data = vec![
            Token::new(String::from("\"")),
            Token::new(String::from("Hello, World!")),
            Token::new(String::from("\"")),
        ];

        let expected = vec![Token::new(String::from("Hello, World!"))];

        let actual = filter_string(data);

        assert_eq!(expected[0].body, actual[0].body);
    }
}
