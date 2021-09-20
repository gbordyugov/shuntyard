// use std::collections::HashMap;
use crate::tokenizer::Token;
use std::rc::Rc;

#[derive(Debug)]
pub enum AstNode {
    Empty,
    Char(char),
    Concat {
        left: Rc<AstNode>,
        right: Rc<AstNode>,
    },
    Alter {
        left: Rc<AstNode>,
        right: Rc<AstNode>,
    },
}

impl AstNode {
    fn from_rpn_tokens(_tokens: Vec<Token>) -> Self {
        unimplemented!()
    }
}
