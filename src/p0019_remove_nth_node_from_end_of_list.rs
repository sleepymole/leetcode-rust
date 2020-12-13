#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

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

    #[test]
    fn test_remove_nth_from_end() {
        assert_eq!(
            Solution::remove_nth_from_end(list!(1, 2, 3, 4, 5), 2),
            list!(1, 2, 3, 5)
        );
        assert_eq!(Solution::remove_nth_from_end(list!(1), 1), list!());
        assert_eq!(Solution::remove_nth_from_end(list!(1, 2), 1), list!(1));
    }
}
