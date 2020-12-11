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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next = &mut head;
        while next.is_some() {
            let mut p = next.as_mut().unwrap().next.take();
            if p.is_some() && next.as_ref().unwrap().val == p.as_ref().unwrap().val {
                while p.is_some() && next.as_ref().unwrap().val == p.as_ref().unwrap().val {
                    p = p.unwrap().next;
                }
                *next = p.take();
            } else {
                next.as_mut().unwrap().next = p.take();
                next = &mut next.as_mut().unwrap().next;
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
    fn test_delete_duplicates() {
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 2, 3, 3, 4, 4, 5])),
            to_list(vec![1, 2, 5])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 1, 2, 3])),
            to_list(vec![2, 3])
        );
    }
}
