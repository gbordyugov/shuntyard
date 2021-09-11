use std::boxed::Box;

enum AstNode {
    Empty,
    Char(char),
    Concat(Box<AstNode>, Box<AstNode>),
    Alter(Box<AstNode>, Box<AstNode>),
}

#[cfg(test)]
mod tests {
    use super::AstNode;

    #[test]
    fn empty_can_be_created() {
        let e = AstNode::Empty;
    }

    #[test]
    fn char_can_be_created() {
        let c = AstNode::Char('a');
    }

    #[test]
    fn concat_can_be_created() {
        let c = AstNode::Concat(Box::new(AstNode::Char('a')), Box::new(AstNode::Char('b')));
    }

    #[test]
    fn alter_can_be_created() {
        let a = AstNode::Alter(Box::new(AstNode::Char('a')), Box::new(AstNode::Char('b')));
    }
}
