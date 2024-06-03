pub struct LL {
    head: Box<Node>,
    tail: Box<Node>,
    pub len: u32,
}

#[derive(Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl LL {
    pub fn new(value: i32) -> Self {
        let node = Box::new(Node { value, next: None });
        Self {
            head: node.to_owned(),
            tail: node,
            len: 1,
        }
    }

    pub fn push(&mut self, value: i32) {
        self.tail = Box::new(Node { value, next: None });
        self.len += 1;
    }

    pub fn push_front(&mut self, value: i32) {
        let node = Node {
            value,
            next: Some(Box::new(*self.head.clone())),
        };
        self.head = Box::new(node);
        self.len += 1;
    }

    pub fn index(&self, index: u32) -> i32 {
        match index {
            0 => self.head.value.clone(),
            x if self.len - 1 > x => {
                let mut next_node = self.head.clone();
                for _ in 0..x {
                    next_node = match next_node.next {
                        Some(x) => x,
                        None => break,
                    }
                }
                next_node.value
            }
            x if self.len - 1 == x => self.tail.value,
            _ => panic!("Index larger than linked list!"),
        }
    }
}
