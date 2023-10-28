#[cfg(test)]
mod tests {
    use crate::{
        lexer::lexer::Lexer,
        shared::token::{Ident, Token},
    };

    #[test]
    fn period_input_success() {
        let lexer = Lexer::new(String::from("."));
        let expected = Token::Period;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

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

    #[test]
    fn plus_input_success() {
        let lexer = Lexer::new(String::from("+"));
        let expected = Token::Plus;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn minus_input_success() {
        let lexer = Lexer::new(String::from("-"));
        let expected = Token::Minus;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn assign_input_success() {
        let lexer = Lexer::new(String::from("="));
        let expected = Token::Assign;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn equality_input_success() {
        let lexer = Lexer::new(String::from("=="));
        let expected = Token::Equals;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bang_input_success() {
        let lexer = Lexer::new(String::from("!"));
        let expected = Token::Bang;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn does_not_equal_input_success() {
        let lexer = Lexer::new(String::from("!="));
        let expected = Token::DoesNotEqual;

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn println_input_success() {
        let lexer = Lexer::new(String::from("println"));
        let expected = Token::Identifier(Ident::Println);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fn_input_success() {
        let lexer = Lexer::new(String::from("fun"));
        let expected = Token::Identifier(Ident::Fun);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn val_input_success() {
        let lexer = Lexer::new(String::from("val"));
        let expected = Token::Identifier(Ident::Val);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn var_input_success() {
        let lexer = Lexer::new(String::from("var"));
        let expected = Token::Identifier(Ident::Var);

        let actual = lexer.next_token().1.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fun_main_input_success() {
        let lexer = Lexer::new("fun main".to_string());

        //[fun] main
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());

        //fun[]main
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());

        //fun [main]
        let (new_lexer, token) = new_lexer.next_token();

        assert_eq!(
            Token::Identifier(Ident::NonIdentifiable("main".to_string())),
            token.unwrap()
        );
    }

    #[test]
    fn fun_main_lp_rp_input_success() {
        let lexer = Lexer::new("fun main()".to_string());
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(Ident::NonIdentifiable("main".to_string())),
            token.unwrap()
        );

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LeftBracket, token.unwrap());

        let (_, token) = new_lexer.next_token();
        assert_eq!(Token::RightBracket, token.unwrap());
    }

    #[test]
    fn fun_main_lp_rp_block_input_success() {
        let lexer = Lexer::new("fun main(){}".to_string());
        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(Ident::NonIdentifiable("main".to_string())),
            token.unwrap()
        );
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LeftBracket, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::RightBracket, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        println!("{:?}", token.clone().unwrap());
        assert_eq!(Token::LeftSquirly, token.unwrap());

        let (_, token) = new_lexer.next_token();
        assert_eq!(Token::RightSquirly, token.unwrap());
    }

    #[test]
    fn fun_main_hello_world_block_input_success() {
        let lexer = Lexer::new("fun main(){println(\"Hello, World!\")}".to_string());

        let (new_lexer, token) = lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Fun), token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Space, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(
            Token::Identifier(Ident::NonIdentifiable("main".to_string())),
            token.unwrap()
        );
        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LeftBracket, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::RightBracket, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LeftSquirly, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::Identifier(Ident::Println), token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::LeftBracket, token.unwrap());

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
        assert_eq!(Token::RightBracket, token.unwrap());

        let (new_lexer, token) = new_lexer.next_token();
        assert_eq!(Token::RightSquirly, token.unwrap());
    }
}
