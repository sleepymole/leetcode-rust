#![allow(dead_code)]

use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let n = self.queue.len();
        self.queue.push_back(x);
        for _ in 0..n {
            let first = self.queue.pop_front().unwrap();
            self.queue.push_back(first);
        }
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_stack() {
        let mut stk = MyStack::new();
        stk.push(1);
        stk.push(2);
        assert_eq!(stk.top(), 2);
        assert_eq!(stk.pop(), 2);
        assert!(!stk.empty());
    }
}
