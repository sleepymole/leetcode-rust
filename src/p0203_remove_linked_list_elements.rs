#![allow(dead_code)]
pub struct Solution {}

use crate::util::ListNode;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next = &mut head;
        while next.is_some() {
            if next.as_ref().unwrap().val == val {
                *next = next.as_mut().unwrap().next.take();
            } else {
                next = &mut next.as_mut().unwrap().next;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        assert_eq!(
            Solution::remove_elements(list!(1, 2, 6, 3, 4, 5, 6), 6),
            list!(1, 2, 3, 4, 5)
        );
    }
}
