#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next = &mut head;
        for _ in 0..m - 1 {
            next = &mut next.as_mut().unwrap().next;
        }
        let mut start = next.take();
        let mut p = &mut start;
        for _ in 0..n - m + 1 {
            p = &mut p.as_mut().unwrap().next;
        }
        let mut prev = p.take();
        while let Some(mut x) = start {
            start = x.next.take();
            x.next = prev;
            prev = Some(x);
        }
        *next = prev;
        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_between() {
        assert_eq!(
            Solution::reverse_between(list!(1, 2, 3, 4, 5), 2, 4),
            list!(1, 4, 3, 2, 5)
        );
    }
}
