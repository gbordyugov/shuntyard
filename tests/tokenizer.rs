#[cfg(test)]
mod tokenize_tests {
    use shuntyard::tokenizer::{tokenize, Op, Token};

    #[test]
    fn empty_string_is_properly_tokenized() {
        let s = "";
        let got = tokenize(s);
        let expected: Vec<Token> = vec![];

        assert_eq!(got, expected);
    }

    #[test]
    fn singleton_is_properly_tokenized() {
        let s = "a";
        let got = tokenize(s);
        let expected = vec![Token::Char('a')];

        assert_eq!(got, expected);
    }

    #[test]
    fn doublet_with_dot_is_properly_tokenized() {
        let s = "ab";
        let got = tokenize(s);
        let expected = vec![Token::Char('a'), Token::Op(Op::Dot), Token::Char('b')];

        assert_eq!(got, expected);
    }

    #[test]
    fn triplet_with_dot_is_properly_tokenized() {
        let s = "abc";
        let got = tokenize(s);
        let expected = vec![
            Token::Char('a'),
            Token::Op(Op::Dot),
            Token::Char('b'),
            Token::Op(Op::Dot),
            Token::Char('c'),
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn doublet_with_alter_is_properly_tokenized() {
        let s = "a|b";
        let got = tokenize(s);
        let expected = vec![Token::Char('a'), Token::Op(Op::Alter), Token::Char('b')];

        assert_eq!(got, expected);
    }

    #[test]
    fn triplet_with_two_alters_is_properly_tokenized() {
        let s = "a|b|c";
        let got = tokenize(s);
        let expected = vec![
            Token::Char('a'),
            Token::Op(Op::Alter),
            Token::Char('b'),
            Token::Op(Op::Alter),
            Token::Char('c'),
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn triplet_with_one_alter_is_properly_tokenized() {
        let s = "ab|c";
        let got = tokenize(s);
        let expected = vec![
            Token::Char('a'),
            Token::Op(Op::Dot),
            Token::Char('b'),
            Token::Op(Op::Alter),
            Token::Char('c'),
        ];

        assert_eq!(got, expected);
    }
}

#[cfg(test)]
mod infix_to_rpn_tests {
    use shuntyard::tokenizer::{infix_to_rpn, Op, Token};

    #[test]
    fn can_convert_empty_vect() {
        let tokens: Vec<Token> = vec![];
        let expected: Vec<Token> = vec![];

        let got = infix_to_rpn(tokens);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_single_char() {
        let tokens: Vec<Token> = vec![Token::Char('a')];
        let expected: Vec<Token> = vec![Token::Char('a')];

        let got = infix_to_rpn(tokens);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_single_dot_operation() {
        let tokens: Vec<Token> = vec![Token::Char('a'), Token::Op(Op::Dot), Token::Char('b')];
        let expected: Vec<Token> = vec![Token::Char('a'), Token::Char('b'), Token::Op(Op::Dot)];

        let got = infix_to_rpn(tokens);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_two_dot_operations() {
        let tokens: Vec<Token> = vec![
            Token::Char('a'),
            Token::Op(Op::Dot),
            Token::Char('b'),
            Token::Op(Op::Dot),
            Token::Char('c'),
        ];
        let expected: Vec<Token> = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Char('c'),
            Token::Op(Op::Dot),
            Token::Op(Op::Dot),
        ];

        let got = infix_to_rpn(tokens);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_dot_and_alter_operations_1() {
        let tokens: Vec<Token> = vec![
            Token::Char('a'),
            Token::Op(Op::Dot),
            Token::Char('b'),
            Token::Op(Op::Alter),
            Token::Char('c'),
        ];
        let expected: Vec<Token> = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Op(Op::Dot),
            Token::Char('c'),
            Token::Op(Op::Alter),
        ];

        let got = infix_to_rpn(tokens);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_dot_and_alter_operations_2() {
        let tokens: Vec<Token> = vec![
            Token::Char('a'),
            Token::Op(Op::Alter),
            Token::Char('b'),
            Token::Op(Op::Dot),
            Token::Char('c'),
        ];
        let expected: Vec<Token> = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Char('c'),
            Token::Op(Op::Dot),
            Token::Op(Op::Alter),
        ];

        let got = infix_to_rpn(tokens);
        assert_eq!(got, expected);
    }
}

#[cfg(test)]
mod string_to_rpn_tests {
    use shuntyard::tokenizer::{string_to_rpn, Op, Token};

    #[test]
    fn can_convert_empty_string() {
        let s = "";
        let expected: Vec<Token> = vec![];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_singleton() {
        let s = "a";
        let expected: Vec<Token> = vec![Token::Char('a')];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_concatenation_of_two() {
        let s = "ab";
        let expected: Vec<Token> = vec![Token::Char('a'), Token::Char('b'), Token::Op(Op::Dot)];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_concatenation_of_three() {
        let s = "abc";
        let expected: Vec<Token> = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Char('c'),
            Token::Op(Op::Dot),
            Token::Op(Op::Dot),
        ];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_alteration_of_two() {
        let s = "a|b";
        let expected: Vec<Token> = vec![Token::Char('a'), Token::Char('b'), Token::Op(Op::Alter)];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_alteration_of_three() {
        let s = "a|b|c";
        let expected: Vec<Token> = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Char('c'),
            Token::Op(Op::Alter),
            Token::Op(Op::Alter),
        ];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_dot_and_alter_operations_1() {
        let s = "a|bc";
        let expected: Vec<Token> = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Char('c'),
            Token::Op(Op::Dot),
            Token::Op(Op::Alter),
        ];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn can_convert_dot_and_alter_operations_2() {
        let s = "ab|c";
        let expected: Vec<Token> = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Op(Op::Dot),
            Token::Char('c'),
            Token::Op(Op::Alter),
        ];

        let got = string_to_rpn(s);
        assert_eq!(got, expected);
    }
}
