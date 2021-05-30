#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    fn list_len(mut l: Option<&Box<ListNode>>) -> usize {
        let mut len = 0;
        while let Some(node) = l {
            l = node.next.as_ref();
            len += 1;
        }
        len
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let len1 = Solution::list_len(l1.as_ref());
        let len2 = Solution::list_len(l2.as_ref());
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = None;
        if len1 < len2 {
            for _ in 0..len2 - len1 {
                let next = l2.as_mut().unwrap().next.take();
                l2.as_mut().unwrap().next = head;
                head = l2;
                l2 = next;
            }
        } else {
            for _ in 0..len1 - len2 {
                let next = l1.as_mut().unwrap().next.take();
                l1.as_mut().unwrap().next = head;
                head = l1;
                l1 = next;
            }
        }
        for _ in 0..len1.min(len2) {
            let next = l1.as_mut().unwrap().next.take();
            l1.as_mut().unwrap().next = head;
            head = l1;
            l1 = next;
            head.as_mut().unwrap().val += l2.as_ref().unwrap().val;
            l2 = l2.as_mut().unwrap().next.take();
        }
        let mut carry = 0;
        let mut head2 = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = head2;
            carry += node.val;
            node.val = carry % 10;
            carry /= 10;
            head2 = Some(node);
        }
        if carry > 0 {
            let mut node = Box::new(ListNode::new(carry));
            node.next = head2;
            head2 = Some(node);
        }
        head2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(list!(7, 2, 4, 3), list!(5, 6, 4)),
            list!(7, 8, 0, 7)
        );
        assert_eq!(
            Solution::add_two_numbers(list!(2, 4, 3), list!(5, 6, 4)),
            list!(8, 0, 7)
        );
        assert_eq!(Solution::add_two_numbers(list!(0), list!(0)), list!(0));
        assert_eq!(
            Solution::add_two_numbers(list!(1, 2, 3), list!(8, 7, 7)),
            list!(1, 0, 0, 0)
        );
    }
}
