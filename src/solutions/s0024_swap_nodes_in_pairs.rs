#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next = &mut head;
        loop {
            if next.is_none() || next.as_ref().unwrap().next.is_none() {
                break;
            }
            let mut second = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = second.as_mut().unwrap().next.take();
            second.as_mut().unwrap().next = next.take();
            *next = second.take();
            next = &mut next.as_mut().unwrap().next.as_mut().unwrap().next;
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
    fn test_swap_pairs() {
        assert_eq!(Solution::swap_pairs(list!(1, 2, 3, 4)), list!(2, 1, 4, 3));
        assert_eq!(Solution::swap_pairs(list!(1)), list!(1));
    }
}
