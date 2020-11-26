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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = head.as_ref();
        while cur.is_some() {
            cur = cur.unwrap().next.as_ref();
            len += 1;
        }
        if n > len {
            return head;
        }
        let mut head = head;
        let mut next = &mut head;
        for _ in 0..len - n {
            next = &mut next.as_mut().unwrap().next;
        }
        *next = next.as_mut().unwrap().next.take();
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
    fn test_remove_nth_from_end() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1]), 1),
            to_list(vec![])
        );
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2]), 1),
            to_list(vec![1])
        );
    }
}
