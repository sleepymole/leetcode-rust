#![allow(dead_code)]

use crate::util::ListNode;

struct Solution {
    n: usize,
    head: Option<Box<ListNode>>,
    seed: usize,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut n = 0;
        let mut p = &head;
        while let Some(node) = p {
            p = &node.next;
            n += 1;
        }
        Solution { n, head, seed: 0 }
    }

    fn get_random(&mut self) -> i32 {
        self.seed = (self.seed * 88054201 + 367) % 1000000007;
        let mut p = &self.head;
        for _ in 0..self.seed % self.n {
            p = &p.as_ref().unwrap().next;
        }
        p.as_ref().unwrap().val
    }
}
