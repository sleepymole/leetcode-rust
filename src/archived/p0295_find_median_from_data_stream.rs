#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if let Some(&Reverse(r)) = self.right.peek() {
            if num < r {
                self.left.push(num);
            } else {
                self.right.push(Reverse(num));
            }
        } else {
            self.left.push(num);
        }
        if self.left.len() > self.right.len() + 1 {
            self.right.push(Reverse(self.left.pop().unwrap()));
        } else if self.left.len() + 1 < self.right.len() {
            self.left.push(self.right.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.left.len() > self.right.len() {
            *self.left.peek().unwrap() as f64
        } else if self.left.len() < self.right.len() {
            (*self.right.peek().unwrap()).0 as f64
        } else if let (Some(&l), Some(&Reverse(r))) = (self.left.peek(), self.right.peek()) {
            (l + r) as f64 / 2.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_finder() {
        let mut finder = MedianFinder::new();
        finder.add_num(1);
        finder.add_num(2);
        assert!((finder.find_median() - 1.5).abs() < f64::EPSILON);
        finder.add_num(3);
        assert!((finder.find_median() - 2.0).abs() < f64::EPSILON);
    }
}
