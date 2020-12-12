#![allow(dead_code)]
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next = &mut head;
        for _ in 0..m - 1 {
            next = &mut next.as_mut().unwrap().next;
        }
        let mut start = next.take();
        let mut p = &mut start;
        for _ in 0..n - m + 1 {
            p = &mut p.as_mut().unwrap().next;
        }
        let mut prev = p.take();
        while let Some(mut x) = start {
            start = x.next.take();
            x.next = prev;
            prev = Some(x);
        }
        *next = prev;
        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut cur = None;
        for &x in v.iter().rev() {
            let mut n = ListNode::new(x);
            n.next = cur;
            cur = Some(Box::new(n));
        }
        cur
    }

    #[test]
    fn test_reverse_between() {
        assert_eq!(
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 2, 4),
            to_list(vec![1, 4, 3, 2, 5])
        );
    }
}
