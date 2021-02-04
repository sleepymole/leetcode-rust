#![allow(dead_code)]
pub struct Solution {}

use crate::util::ListNode;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = None;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = tail;
            tail = Some(node);
        }
        tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        assert_eq!(
            Solution::reverse_list(list!(1, 2, 3, 4, 5)),
            list!(5, 4, 3, 2, 1)
        );
    }
}
