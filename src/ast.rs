use std::rc::Rc;

enum AstNode {
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
    pub fn char(c: char) -> Self {
        AstNode::Char(c)
    }

    pub fn concat(left: Rc<AstNode>, right: Rc<AstNode>) -> Self {
        AstNode::Concat { left, right }
    }

    pub fn alter(left: Rc<AstNode>, right: Rc<AstNode>) -> Self {
        AstNode::Alter { left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::AstNode;
    use super::Rc;

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
