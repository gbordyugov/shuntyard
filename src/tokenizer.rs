#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Alter,
    Dot,
    LeftParen,
    RightParen,
}

impl Op {
    fn prec(&self) -> i8 {
        match self {
            Op::Alter => 0,
            Op::Dot => 1,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Char(char),
    Op(Op),
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();

    if s.is_empty() {
        return output;
    }

    let mut raw_tokens: Vec<Token> = Vec::new();

    for c in s.chars() {
        let t = match c {
            '(' => Token::Op(Op::LeftParen),
            ')' => Token::Op(Op::RightParen),
            '|' => Token::Op(Op::Alter),
            _ => Token::Char(c),
        };
        raw_tokens.push(t)
    }

    output.push(raw_tokens[0]);

    for i in 0..raw_tokens.len() - 1 {
        let this = raw_tokens[i];
        let next = raw_tokens[i + 1];

        if let (Token::Char(_), Token::Char(_)) = (this, next) {
            output.push(Token::Op(Op::Dot));
        }
        output.push(next);
    }

    output
}

pub fn infix_to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut oper_stack: Vec<Op> = Vec::new();

    let mut tokens_iter = tokens.iter();

    for token in tokens_iter {
        match token {
            Token::Char(_) => output.push(*token),
            Token::Op(op1) => {
                while let Some(op2) = oper_stack.last() {
                    if op2.prec() <= op1.prec() {
                        break;
                    }
                    output.push(Token::Op(*op2));
                    oper_stack.pop();
                }
                oper_stack.push(*op1);
            }
        }
    }

    while let Some(op) = oper_stack.pop() {
        output.push(Token::Op(op))
    }

    output
}
