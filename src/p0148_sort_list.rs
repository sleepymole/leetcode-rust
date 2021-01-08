#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            cur = &node.next;
            n += 1;
        }
        if n <= 1 {
            return head;
        }
        let mut head = head;
        let mut right = &mut head;
        for _ in 0..n / 2 {
            right = &mut right.as_mut().unwrap().next;
        }
        let right = right.take();
        let mut left = Solution::sort_list(head);
        let mut right = Solution::sort_list(right);
        let mut head = None;
        let mut cur = &mut head;
        while left.is_some() && right.is_some() {
            if left.as_ref().unwrap().val <= right.as_ref().unwrap().val {
                *cur = left;
                left = cur.as_mut().unwrap().next.take();
            } else {
                *cur = right;
                right = cur.as_mut().unwrap().next.take();
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        if left.is_some() {
            *cur = left;
        }
        if right.is_some() {
            *cur = right;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_list() {
        assert_eq!(
            Solution::sort_list(list!(4, 2, 1, 3)),
            list!(1, 2, 3, 4)
        );
        assert_eq!(
            Solution::sort_list(list!(-1, 5, 3, 4, 0)),
            list!(-1, 0, 3, 4, 5)
        );
    }
}
