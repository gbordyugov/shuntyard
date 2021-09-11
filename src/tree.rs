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

    pub fn fork(val: i32, left: Node, right: Node) -> Self {
        Node::Fork {
            val: val,
            left: Rc::new(left),
            right: Rc::new(right),
        }
    }
}
