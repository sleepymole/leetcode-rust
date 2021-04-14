#![allow(dead_code)]
pub struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Entry {
    val: i32,
    count: usize,
    index: Option<usize>,
}

impl Entry {
    fn new(val: i32) -> Self {
        Entry {
            val,
            count: 1,
            index: None,
        }
    }
}

struct Recorder {
    k: usize,
    nums: HashMap<i32, Rc<RefCell<Entry>>>,
    heap: Vec<Rc<RefCell<Entry>>>,
}

impl Recorder {
    fn new(k: usize) -> Self {
        Recorder {
            k,
            nums: HashMap::new(),
            heap: Vec::new(),
        }
    }

    fn record(&mut self, val: i32) {
        let entry = if let Some(entry) = self.nums.get(&val) {
            entry.borrow_mut().count += 1;
            entry.clone()
        } else {
            let entry = Rc::new(RefCell::new(Entry::new(val)));
            self.nums.insert(val, entry.clone());
            entry
        };
        if self.heap.is_empty() {
            entry.borrow_mut().index = Some(0);
            self.heap.push(entry);
            return;
        }
        if entry.borrow().index.is_some() {
            let i = entry.borrow().index.unwrap();
            self.shiftdown(i);
            self.shiftup(i);
            return;
        }
        if self.heap.len() >= self.k {
            if entry.borrow().count <= self.count(0) {
                return;
            };
            self.pop();
        }
        self.push(entry);
    }

    fn push(&mut self, entry: Rc<RefCell<Entry>>) {
        entry.borrow_mut().index = Some(self.heap.len());
        self.heap.push(entry);
        self.shiftup(self.heap.len() - 1);
    }

    fn pop(&mut self) {
        self.swap(0, self.heap.len() - 1);
        self.heap.last().unwrap().borrow_mut().index = None;
        self.heap.pop();
        self.shiftdown(0);
    }

    fn count(&self, i: usize) -> usize {
        self.heap[i].borrow().count
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.count(i) < self.count(j)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.heap[i].borrow_mut().index = Some(i);
        self.heap[j].borrow_mut().index = Some(j);
    }

    fn shiftup(&mut self, mut i: usize) {
        while i > 0 {
            let j = (i - 1) / 2;
            if !self.less(i, j) {
                break;
            }
            self.swap(i, j);
            i = j;
        }
    }

    fn shiftdown(&mut self, mut i: usize) {
        loop {
            let mut j = 2 * i + 1;
            if j >= self.heap.len() {
                break;
            }
            if j + 1 < self.heap.len() && self.less(j + 1, j) {
                j += 1;
            }
            if !self.less(j, i) {
                break;
            }
            self.swap(i, j);
            i = j
        }
    }

    fn topk(&self) -> Vec<i32> {
        let mut heap = self.heap.clone();
        heap.sort_unstable_by(|a, b| b.borrow().count.cmp(&a.borrow().count));
        heap.into_iter().map(|e| e.borrow().val).collect()
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut r = Recorder::new(k as usize);
        for x in nums {
            r.record(x);
        }
        r.topk()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums
    }

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            sorted(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
