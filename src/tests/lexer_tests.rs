#[cfg(test)]
mod lexer_symbol_tests {
    use crate::{
        lexer::lexer::Lexer,
        shared::token::{Ident, Token},
    };

    #[test]
    fn colon_success() {
        let lexer = Lexer::new(String::from(":"));
        let expected = Token::Colon;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn l_angle_brack_success() {
        let lexer = Lexer::new(String::from("<"));
        let expected = Token::LAngleBracket;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn r_angle_brack_success() {
        let lexer = Lexer::new(String::from(">"));
        let expected = Token::RAngleBracket;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn period_success() {
        let lexer = Lexer::new(String::from("."));
        let expected = Token::Period;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn space_success() {
        let lexer = Lexer::new(String::from(" "));
        let expected = Token::Space;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lb_success() {
        let lexer = Lexer::new(String::from("("));
        let expected = Token::LParen;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn main_success() {
        let lexer = Lexer::new(String::from("main"));
        let expected = Token::Identifier(Ident::Main);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn rb_success() {
        let lexer = Lexer::new(String::from(")"));
        let expected = Token::RParen;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn ls_success() {
        let lexer = Lexer::new(String::from("{"));
        let expected = Token::LCurlyBrace;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn rs_success() {
        let lexer = Lexer::new(String::from("}"));
        let expected = Token::RCurlyBrace;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn comma_success() {
        let lexer = Lexer::new(String::from(","));
        let expected = Token::Comma;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn quote_success() {
        let lexer = Lexer::new(String::from("\'"));
        let expected = Token::Quote;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn double_quote_success() {
        let lexer = Lexer::new(String::from("\""));
        let expected = Token::Quote;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn plus_success() {
        let lexer = Lexer::new(String::from("+"));
        let expected = Token::Plus;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn minus_success() {
        let lexer = Lexer::new(String::from("-"));
        let expected = Token::Minus;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiply_success() {
        let lexer = Lexer::new(String::from("*"));
        let expected = Token::Multiply;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn divide_success() {
        let lexer = Lexer::new(String::from("/"));
        let expected = Token::Divide;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn assign_success() {
        let lexer = Lexer::new(String::from("="));
        let expected = Token::Assign;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn equality_success() {
        let lexer = Lexer::new(String::from("=="));
        let expected = Token::Equals;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bang_success() {
        let lexer = Lexer::new(String::from("!"));
        let expected = Token::Bang;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn question_success() {
        let lexer = Lexer::new(String::from("?"));
        let expected = Token::Question;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn safecall_success() {
        let lexer = Lexer::new(String::from("?."));
        let expected = Token::Safecall;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn does_not_equal_success() {
        let lexer = Lexer::new(String::from("!="));
        let expected = Token::DoesNotEqual;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn println_success() {
        let lexer = Lexer::new(String::from("println"));
        let expected = Token::Identifier(Ident::NonIdentifiable(String::from("println")));

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fn_success() {
        let lexer = Lexer::new(String::from("fun"));
        let expected = Token::Identifier(Ident::Fun);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn val_success() {
        let lexer = Lexer::new(String::from("val"));
        let expected = Token::Identifier(Ident::Val);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn var_success() {
        let lexer = Lexer::new(String::from("var"));
        let expected = Token::Identifier(Ident::Var);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fun_main_success() {
        let lexer = Lexer::new("fun main".to_string());

        //[fun] main
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());

        //fun[]main
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());

        //fun [main]
        let (_new_lexer, token) = new_lexer.next_token();

        assert_eq!(Token::Identifier(Ident::Main), token.unwrap());
    }

    #[test]
    fn fun_main_lp_rp_success() {
        let lexer = Lexer::new("fun main()".to_string());
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Main), token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LParen, token.unwrap());

        let (_, token) = new_lexer.next_token();
        assert_eq!(Token::RParen, token.unwrap());
    }

    #[test]
    fn fun_main_lp_rp_block_success() {
        let lexer = Lexer::new("fun main(){}".to_string());
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Main), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LParen, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::RParen, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        println!("{:?}", token.clone().unwrap());
        assert_eq!(Token::LCurlyBrace, token.unwrap());

        let (_, token) = new_lexer.next_token();
        assert_eq!(Token::RCurlyBrace, token.unwrap());
    }

    #[test]
    fn fun_main_hello_world_block_success() {
        let lexer = Lexer::new("fun main(){println(\"Hello, World!\")}".to_string());

        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Main), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LParen, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::RParen, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LCurlyBrace, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(Ident::NonIdentifiable(String::from("println"))),
            token.unwrap()
        );

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LParen, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Quote, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(Ident::NonIdentifiable("Hello".to_string())),
            token.unwrap()
        );

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Comma, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(Ident::NonIdentifiable("World".to_string())),
            token.unwrap()
        );

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Bang, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Quote, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::RParen, token.unwrap());

        let (_new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::RCurlyBrace, token.unwrap());
    }
}

#[cfg(test)]
mod iterator_method_tests {
    use crate::{
        lexer::lexer::Lexer,
        shared::token::{Ident, Token},
    };

    #[test]
    fn collect_success() {
        let data = "fun main()";

        let expected = vec![
            Token::Identifier(Ident::Fun),
            Token::Space,
            Token::Identifier(Ident::Main),
            Token::LParen,
            Token::RParen,
        ];

        let actual = Lexer::new(data.to_string()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn collect_hello_world_success() {
        let lexer = Lexer::new("fun main(){println(\"Hello, World!\")}".to_string());

        let expected = vec![
            Token::Identifier(Ident::Fun),
            Token::Space,
            Token::Identifier(Ident::Main),
            Token::LParen,
            Token::RParen,
            Token::LCurlyBrace,
            Token::Identifier(Ident::NonIdentifiable(String::from("println"))),
            Token::LParen,
            Token::Quote,
            Token::Identifier(Ident::NonIdentifiable("Hello".to_string())),
            Token::Comma,
            Token::Space,
            Token::Identifier(Ident::NonIdentifiable("World".to_string())),
            Token::Bang,
            Token::Quote,
            Token::RParen,
            Token::RCurlyBrace,
        ];

        let actual = lexer.collect();

        assert_eq!(expected, actual);
    }
}
