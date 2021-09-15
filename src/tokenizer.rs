#[derive(Clone, Copy, Debug)]
pub enum Token {
    Char(char),
    Dot,
    LeftParen,
    RightParen,
    Alter,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();

    if s.len() < 1 {
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

    for i in 0..raw_tokens.len() - 1 {
        output.push(raw_tokens[i]);
        if let Token::Char(_) = raw_tokens[i + 1] {
            output.push(Token::Dot);
        }
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
        let expected: Vec<Token> = Vec::new();
        assert_eq!(got.len(), expected.len());
    }
}
