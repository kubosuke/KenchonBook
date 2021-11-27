const N: usize = 3;

struct Heap {
    heap: Vec<usize>,
}

impl Heap {
    fn new() -> Self {
        Heap { heap: vec![] }
    }

    fn push(&mut self, x: usize) {
        self.heap.push(x);
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let p = (i - 1) / 2;
            if self.heap[p] >= x {
                break;
            }
            self.heap[i] = self.heap[p];
            i = p;
        }
        self.heap[i] = x;
    }

    fn top(&mut self) -> Option<usize> {
        if !self.heap.is_empty() {
            return Some(self.heap[0]);
        };
        None
    }

    fn pop(&mut self) {
        if self.heap.is_empty() {
            return;
        };
        let x = self.heap.pop().unwrap();

        let mut i = 0;
        while (i * 2) + 1 < self.heap.len() {
            let mut child1 = i * 2 + 1;
            let child2 = i * 2 + 2;
            if child2 < self.heap.len() && self.heap[child2] > self.heap[child1] {
                child1 = child2;
            }
            if self.heap[child1] <= x {
                break;
            }
            self.heap[i] = self.heap[child1];
            i = child1;
        }
        self.heap[i] = x;
    }
}

fn main() {
    let mut heap = Heap::new();
    heap.push(5);
    heap.push(6);
    heap.push(1);
    heap.push(2);
    heap.push(7);
    heap.push(3);
    heap.push(4);
    assert_eq!("[7, 6, 4, 2, 5, 1, 3]", format!("{:?}", heap.heap))
}
