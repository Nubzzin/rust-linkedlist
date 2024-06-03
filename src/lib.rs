pub mod ll {
    pub struct LinkedL {
        head: Box<Node>,
        tail: Option<Box<Node>>,
        len: u32,
    }

    #[derive(Clone)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    impl LinkedL {
        pub fn new(value: i32) -> Self {
            Self {
                head: Box::new(Node { value, next: None }),
                tail: None,
                len: 1,
            }
        }

        pub fn push(&mut self, value: i32) {
            let node = Some(Box::new(Node { value, next: None }));
            self.tail = node;
            self.len += 1;
        }

        pub fn push_front(&mut self, value: i32) {
            let node = Box::new(Node {
                value,
                next: Some(Box::new(*self.head.clone())),
            });
            self.head = node;
            self.len += 1;
        }

        pub fn index(&self, index: u32) -> i32 {
            match index {
                0 => self.head.value.clone(),
                x if self.len - 1 >= x => {
                    let mut next_node = self.head.clone();
                    for _ in 0..x {
                        next_node = match next_node.next {
                            Some(x) => x,
                            None => break,
                        }
                    }
                    next_node.value
                }
                _ => panic!("Index larger than linked list!"),
            }
        }
    }
}
