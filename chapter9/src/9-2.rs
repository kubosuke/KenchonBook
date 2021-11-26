use std::collections::LinkedList;

struct Stack {
    linked_list: LinkedList<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            linked_list: LinkedList::new(),
        }
    }

    fn pop(&mut self) -> Option<i32> {
        return self.linked_list.pop_front();
    }

    fn push(&mut self, x: i32) {
        self.linked_list.push_front(x)
    }
}

fn main() {
    let reverse_polish_notation: Vec<&str> = vec!["3", "4", "+", "1", "2", "-", "*"];
    let mut stack = Stack::new();
    let mut ans = 0;

    for rpn in reverse_polish_notation {
        match rpn.parse::<i32>() {
            Ok(r) => stack.push(r),
            _ => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match rpn {
                    "+" => {
                        ans = a + b;
                        stack.push(ans);
                    }
                    "-" => {
                        ans = a - b;
                        stack.push(ans)
                    }
                    "*" => {
                        ans = a * b;
                        stack.push(ans)
                    }
                    "/" => {
                        ans = a / b;
                        stack.push(ans)
                    }
                    _ => (),
                }
            }
        }
    }
    assert_eq!(-7, ans);
}
