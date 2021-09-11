use std::rc::Rc;

enum AstNode {
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
    pub fn empty() -> Self {
        AstNode::Empty
    }

    pub fn char(c: char) -> Self {
        AstNode::Char(c)
    }
    /*
    pub fn concat(left: &AstNode, right: &AstNode) -> Self {
        AstNode::Concat {
            left: left,
            right: right,
        };
    }
    */
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
            left: Rc::new(AstNode::Char('a')),
            right: Rc::new(AstNode::Char('b')),
        };
    }

    #[test]
    fn alter_can_be_created() {
        let _a = AstNode::Alter {
            left: Rc::new(AstNode::Char('a')),
            right: Rc::new(AstNode::Char('b')),
        };
    }
}
