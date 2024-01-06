#[cfg(test)]
mod test {
    #[test]
    fn test_string_interpolation() {
        use crate::lexer::Lexer;

        let source = "\"outer start {\"inner start {expr_1 + expr_2} inner end\"} outer end\"";

        let mut lexer = Lexer::new(source);

        let (token_names, token_lexemes) = lexer.get_token_names_and_lexemes_vec();

        assert_eq!(
            token_names,
            vec![
                String::from("string start"),
                String::from("string literal"),
                String::from("interpolation start"),
                String::from("string start"),
                String::from("string literal"),
                String::from("interpolation start"),
                String::from("identifier"),
                String::from("plus"),
                String::from("identifier"),
                String::from("interpolation end"),
                String::from("string literal"),
                String::from("string end"),
                String::from("interpolation end"),
                String::from("string literal"),
                String::from("string end"),
                String::from("end of file")
            ]
        );

        assert_eq!(
            token_lexemes,
            vec![
                String::from("\""),
                String::from("outer start "),
                String::from("{"),
                String::from("\""),
                String::from("inner start "),
                String::from("{"),
                String::from("expr_1"),
                String::from("+"),
                String::from("expr_2"),
                String::from("}"),
                String::from(" inner end"),
                String::from("\""),
                String::from("}"),
                String::from(" outer end"),
                String::from("\""),
                String::from("")
            ]
        );
    }

    #[test]
    fn test_numbers() {
        use crate::lexer::Lexer;

        let source = "1 + 3 * 4 / 2 - 1";

        let mut lexer = Lexer::new(source);

        let (token_names, token_lexemes) = lexer.get_token_names_and_lexemes_vec();

        assert_eq!(
            token_names,
            vec![
                String::from("number literal"),
                String::from("plus"),
                String::from("number literal"),
                String::from("multiply"),
                String::from("number literal"),
                String::from("divide"),
                String::from("number literal"),
                String::from("minus"),
                String::from("number literal"),
                String::from("end of file")
            ]
        );

        assert_eq!(
            token_lexemes,
            vec![
                String::from("1"),
                String::from("+"),
                String::from("3"),
                String::from("*"),
                String::from("4"),
                String::from("/"),
                String::from("2"),
                String::from("-"),
                String::from("1"),
                String::from("")
            ]
        );
    }
}
