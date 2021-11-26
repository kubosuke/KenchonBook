use std::collections::LinkedList;
use std::fmt;

struct Stack {
    linked_list: LinkedList<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            linked_list: LinkedList::new(),
        }
    }

    fn pop(&mut self) -> Option<usize> {
        return self.linked_list.pop_front();
    }

    fn push(&mut self, x: usize) {
        self.linked_list.push_front(x)
    }

    fn is_empty(&self) -> bool {
        self.linked_list.is_empty()
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stack")
            .field("linked_list", &self.linked_list)
            .finish()
    }
}

fn main() {
    let q = "(()(())())(()())";

    let n = q.chars().count();
    let mut stack = Stack::new();
    let mut ch = q.chars();
    let mut pairs: Vec<(usize, usize)> = vec![];

    for i in 0..n {
        match ch.next() {
            Some(c) if c == '(' => {
                stack.push(i+1); // 1-index
            },
            Some(c) if c == ')' => {
                match stack.pop() {
                    Some(p) => pairs.push((p, i+1)), // 1-index
                    _ => {
                        println!("No");
                        return
                    }
                }
            }
            _ => {
                if !stack.is_empty() {
                    println!("No");
                    return
                };
            }
        }
    }

    // [(2, 3), (5, 6), (4, 7), (8, 9), (1, 10), (12, 13), (14, 15), (11, 16)]
    println!("{:?}", pairs)
}
