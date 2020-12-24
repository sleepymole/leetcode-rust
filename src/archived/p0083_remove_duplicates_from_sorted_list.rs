#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut p = &mut head;
        while p.is_some() {
            let mut next = p.as_mut().unwrap().next.take();
            while next.is_some() && next.as_ref().unwrap().val == p.as_ref().unwrap().val {
                next = next.unwrap().next;
            }
            p.as_mut().unwrap().next = next.take();
            p = &mut p.as_mut().unwrap().next;
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
    fn test_delete_duplicates() {
        assert_eq!(Solution::delete_duplicates(list!(1, 1, 2)), list!(1, 2));
        assert_eq!(
            Solution::delete_duplicates(list!(1, 1, 2, 3, 3)),
            list!(1, 2, 3)
        );
    }
}
