#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut left, mut right) = (None, None);
        let mut left_next = &mut left;
        let mut right_next = &mut right;
        let mut i = 0;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if i % 2 == 0 {
                *left_next = Some(node);
                left_next = &mut left_next.as_mut().unwrap().next;
            } else {
                *right_next = Some(node);
                right_next = &mut right_next.as_mut().unwrap().next;
            }
            i += 1;
        }
        *left_next = right.take();
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_even_list() {
        assert_eq!(
            Solution::odd_even_list(list!(1, 2, 3, 4, 5)),
            list!(1, 3, 5, 2, 4)
        );
        assert_eq!(
            Solution::odd_even_list(list!(2, 1, 3, 5, 6, 4, 7)),
            list!(2, 3, 6, 7, 1, 5, 4)
        );
    }
}
