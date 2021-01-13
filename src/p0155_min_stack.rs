#![allow(dead_code)]

struct MinStack {
    min: Vec<i32>,
    nums: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            min: vec![],
            nums: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.nums.push(x);
        if self.min.is_empty() {
            self.min.push(x);
        } else {
            self.min.push(x.min(*self.min.last().unwrap()));
        }
    }

    fn pop(&mut self) {
        self.min.pop();
        self.nums.pop();
    }

    fn top(&self) -> i32 {
        *self.nums.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut mstk = MinStack::new();
        mstk.push(-2);
        mstk.push(0);
        mstk.push(-3);
        assert_eq!(mstk.get_min(), -3);
        mstk.pop();
        mstk.top();
        assert_eq!(mstk.get_min(), -2);
    }
}
