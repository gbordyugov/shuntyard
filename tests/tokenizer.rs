#[cfg(test)]
mod tests {
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
