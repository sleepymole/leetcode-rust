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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left: Option<Box<ListNode>> = None;
        let mut right: Option<Box<ListNode>> = None;
        let mut lnext = &mut left;
        let mut rnext = &mut right;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                *lnext = Some(node);
                lnext = &mut lnext.as_mut().unwrap().next;
            } else {
                *rnext = Some(node);
                rnext = &mut rnext.as_mut().unwrap().next;
            }
        }
        *lnext = right;
        left
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
    fn test_partition() {
        assert_eq!(
            Solution::partition(to_list(vec![1, 4, 3, 2, 5, 2]), 3),
            to_list(vec![1, 2, 2, 4, 3, 5])
        );
    }
}
