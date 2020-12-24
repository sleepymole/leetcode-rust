#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut n = 0_i32;
        let mut p = &head;
        while p.is_some() {
            n += 1;
            p = &p.as_ref().unwrap().next;
        }
        let k = k % n;
        if k == 0 {
            return head;
        }
        let mut head = head;
        let mut p = &mut head;
        for _ in 0..(n - k - 1) {
            p = &mut p.as_mut().unwrap().next;
        }
        let mut head_new = p.as_mut().unwrap().next.take();
        let mut next = &mut head_new;
        while next.is_some() {
            next = &mut next.as_mut().unwrap().next;
        }
        *next = head;
        head_new
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
    fn test_rotate_right() {
        assert_eq!(
            Solution::rotate_right(list!(1, 2, 3, 4, 5), 2),
            list!(4, 5, 1, 2, 3)
        );
        assert_eq!(Solution::rotate_right(list!(0, 1, 2), 4), list!(2, 0, 1));
    }
}
