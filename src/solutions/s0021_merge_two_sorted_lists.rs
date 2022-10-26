#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut next = &mut head;
        let (mut l1, mut l2) = (l1, l2);
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let l1_next = l1.as_mut().unwrap().next.take();
                *next = l1.take();
                l1 = l1_next;
            } else {
                let l2_next = l2.as_mut().unwrap().next.take();
                *next = l2.take();
                l2 = l2_next;
            }
            next = &mut next.as_mut().unwrap().next;
        }
        if l1.is_some() {
            *next = l1;
        }
        if l2.is_some() {
            *next = l2;
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
    fn test_merge_two_lists() {
        assert_eq!(
            Solution::merge_two_lists(list!(1, 2, 4), list!(1, 3, 4)),
            list!(1, 1, 2, 3, 4, 4)
        );
        assert_eq!(Solution::merge_two_lists(list!(), list!()), list!());
        assert_eq!(Solution::merge_two_lists(list!(), list!(0)), list!(0));
    }
}
