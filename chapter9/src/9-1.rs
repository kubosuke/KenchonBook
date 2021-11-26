use std::collections::LinkedList;

struct Stack {
    linked_list: LinkedList<i32>
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            linked_list: LinkedList::new()
        }
    }

    fn pop(&mut self) -> Option<i32> {
        return self.linked_list.pop_front()
    }

    fn push(&mut self, x: i32) {
        self.linked_list.push_front(x)
    }
}

struct Queue {
    linked_list: LinkedList<i32>
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            linked_list: LinkedList::new()
        }
    }

    fn pop(&mut self) -> Option<i32> {
        return self.linked_list.pop_back()
    }

    fn push(&mut self, x: i32) {
        self.linked_list.push_front(x)
    }
}


fn main() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(5);
    stack.push(4);
    stack.push(1);

    assert_eq!(1, stack.pop().unwrap());
    assert_eq!(4, stack.pop().unwrap());
    assert_eq!(5, stack.pop().unwrap());
    assert_eq!(3, stack.pop().unwrap());
    assert_eq!(None, stack.pop());

    let mut queue = Queue::new();
    queue.push(3);
    queue.push(5);
    queue.push(4);
    queue.push(1);

    assert_eq!(3, queue.pop().unwrap());
    assert_eq!(5, queue.pop().unwrap());
    assert_eq!(4, queue.pop().unwrap());
    assert_eq!(1, queue.pop().unwrap());
    assert_eq!(None, queue.pop());
}
