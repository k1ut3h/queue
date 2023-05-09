#[derive(Debug, Clone)]
struct QNode {
    value: i32,
    next: Option<Box<QNode>>,
}

#[derive(Debug, Clone)]
struct Queue {
    length: u32,
    head: Option<&'static QNode>,
    tail: Option<&'static QNode>,
}

impl Queue {
    fn new() -> Queue {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }
    fn enqueue(&mut self, item: i32) {
        let node = QNode {
            value: item,
            next: None,
        };
        if self.tail.is_none() {
            self.tail = Some(&node.clone());
            self.head = Some(&node.clone());
        }
    }
    fn deque(&mut self) -> Option<&i32> {
        if self.head.is_some() {
            self.length -= 1;
            let h = self.head;
            self.head = match &self.head?.next {
                Some(val) => Some(val),
                None => None,
            };
            Some(&h?.value)
        } else {
            None
        }
    }
    fn peek(&self) -> Option<&i32> {
        if let Some(head) = &self.head {
            Some(&head.value)
        } else {
            None
        }
    }
}

fn main() {
    let queue = Queue::new();
    println!("{:?}", queue);
}
