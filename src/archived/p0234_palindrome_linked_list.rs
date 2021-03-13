#![allow(dead_code)]
pub struct Solution {}

use crate::util::ListNode;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut p = &head;
        let mut n = 0;
        while let Some(node) = p {
            p = &node.next;
            n += 1;
        }
        if n <= 1 {
            return true;
        }
        let mut left = head;
        let mut cur = &mut left;
        for _ in 0..(n + 1) / 2 {
            cur = &mut cur.as_mut().unwrap().next;
        }
        let mut cur = cur.take();
        let mut right = None;
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = right;
            right = Some(node);
        }
        let mut ok = true;
        let mut p = &mut left;
        let mut cur = right.take();
        while let Some(mut node) = cur {
            ok &= p.as_ref().unwrap().val == node.val;
            p = &mut p.as_mut().unwrap().next;
            cur = node.next.take();
            node.next = right;
            right = Some(node);
        }
        if n % 2 == 1 {
            p = &mut p.as_mut().unwrap().next;
        }
        *p = right;
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(list!(1, 2)), false);
        assert_eq!(Solution::is_palindrome(list!(1, 2, 2, 1)), true);
        assert_eq!(Solution::is_palindrome(list!(1, 2, 3, 2, 1)), true);
    }
}
