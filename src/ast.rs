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
