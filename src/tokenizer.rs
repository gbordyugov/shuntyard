#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Char(char),
    Dot,
    Alter,
    LeftParen,
    RightParen,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();

    if s.is_empty() {
        return output;
    }

    let mut raw_tokens: Vec<Token> = Vec::new();

    for c in s.chars() {
        let t = match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '|' => Token::Alter,
            _ => Token::Char(c),
        };
        raw_tokens.push(t)
    }

    output.push(raw_tokens[0]);

    for i in 0..raw_tokens.len() - 1 {
        let this = raw_tokens[i];
        let next = raw_tokens[i + 1];

        if let (Token::Char(_), Token::Char(_)) = (this, next) {
            output.push(Token::Dot);
        }
        output.push(next);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::tokenize;
    use super::Token;

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
        let expected = vec![Token::Char('a'), Token::Dot, Token::Char('b')];

        assert_eq!(got, expected);
    }

    #[test]
    fn triplet_with_dot_is_properly_tokenized() {
        let s = "abc";
        let got = tokenize(s);
        let expected = vec![
            Token::Char('a'),
            Token::Dot,
            Token::Char('b'),
            Token::Dot,
            Token::Char('c'),
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn doublet_with_alter_is_properly_tokenized() {
        let s = "a|b";
        let got = tokenize(s);
        let expected = vec![Token::Char('a'), Token::Alter, Token::Char('b')];

        assert_eq!(got, expected);
    }

    #[test]
    fn triplet_with_two_alters_is_properly_tokenized() {
        let s = "a|b|c";
        let got = tokenize(s);
        let expected = vec![
            Token::Char('a'),
            Token::Alter,
            Token::Char('b'),
            Token::Alter,
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
            Token::Dot,
            Token::Char('b'),
            Token::Alter,
            Token::Char('c'),
        ];

        assert_eq!(got, expected);
    }
}
