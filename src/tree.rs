use std::rc::Rc;

enum Node {
    Nil,
    Fork {
        val: i32,
        left: Rc<Node>,
        right: Rc<Node>,
    },
}

impl Node {
    pub fn empty() -> Self {
        Node::Nil
    }

    pub fn fork(val: i32, left: Rc<Node>, right: Rc<Node>) -> Self {
        Node::Fork { val, left, right }
    }
}
