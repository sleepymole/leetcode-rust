#![allow(dead_code)]

struct MyQueue {
    stk1: Vec<i32>,
    stk2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stk1: Vec::new(),
            stk2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(v) = self.stk1.pop() {
            self.stk2.push(v);
        }
        self.stk1.push(x);
        while let Some(v) = self.stk2.pop() {
            self.stk1.push(v);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stk1.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        *self.stk1.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stk1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }
}
