#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next = &mut head;
        loop {
            let mut start = next.take();
            let mut prev = &mut start;
            for _ in 0..k - 1 {
                if prev.is_none() {
                    break;
                }
                prev = &mut prev.as_mut().unwrap().next;
            }
            if prev.is_none() {
                *next = start;
                break;
            }
            let mut prev = prev.as_mut().unwrap().next.take();
            while let Some(mut x) = start {
                start = x.next.take();
                x.next = prev;
                prev = Some(x);
            }
            *next = prev;
            for _ in 0..k {
                next = &mut next.as_mut().unwrap().next;
            }
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
    fn test_reverse_k_group() {
        assert_eq!(
            Solution::reverse_k_group(list!(1, 2, 3, 4, 5), 2),
            list!(2, 1, 4, 3, 5)
        );
        assert_eq!(
            Solution::reverse_k_group(list!(1, 2, 3, 4, 5), 3),
            list!(3, 2, 1, 4, 5)
        );
        assert_eq!(
            Solution::reverse_k_group(list!(1, 2, 3, 4, 5), 1),
            list!(1, 2, 3, 4, 5)
        );
        assert_eq!(Solution::reverse_k_group(list!(1), 1), list!(1));
    }
}
