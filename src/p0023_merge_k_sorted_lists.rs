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

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut heap = BinaryHeap::new();
        while let Some(head) = lists.pop() {
            if head.is_none() {
                continue;
            }
            heap.push(head.unwrap());
        }
        let mut head: Option<Box<ListNode>> = None;
        let mut next = &mut head;
        while let Some(mut x) = heap.pop() {
            let x2 = x.next.take();
            *next = Some(x);
            next = &mut next.as_mut().unwrap().next;
            if x2.is_some() {
                heap.push(x2.unwrap());
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
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
    fn test_merge_k_lists() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), to_list(vec![]));
        assert_eq!(
            Solution::merge_k_lists(vec![to_list(vec![])]),
            to_list(vec![])
        );
    }
}
