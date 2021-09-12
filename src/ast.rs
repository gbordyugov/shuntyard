use std::rc::Rc;
use std::collections::HashMap;

/**
 * "abcd|efgh" => Alter("abcd", "efgh")
 * "abc(d|e)fgh" -> Concat("abc", Concat(Alter("d", "e"), "fgh"))
 */

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
    fn from_str(s: &str) -> Self {
        let mut precedence: HashMap<char, u8> = HashMap::new();
        precedence.insert('|', 1);
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::AstNode;
    use super::Rc;

    #[test]
    fn empty_can_be_created() {
        let _e = AstNode::Empty;
    }

    #[test]
    fn char_can_be_created() {
        let _c = AstNode::Char('a');
    }

    #[test]
    fn concat_can_be_created() {
        let _c = AstNode::Concat {
            left: Rc::new(AstNode::Char('l')),
            right: Rc::new(AstNode::Char('r')),
        };
    }

    #[test]
    fn alter_can_be_created() {
        let _a = AstNode::Alter {
            left: Rc::new(AstNode::Char('l')),
            right: Rc::new(AstNode::Char('r')),
        };
    }
}
