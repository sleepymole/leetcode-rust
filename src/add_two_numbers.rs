#![allow(dead_code)]
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut head = None;
        let mut tail = &mut head;
        let mut carry = 0;
        loop {
            let mut x = carry;
            let mut exit = true;
            x += match l1 {
                Some(node) => {
                    l1 = node.next;
                    exit = false;
                    node.val
                }
                None => 0,
            };
            x += match l2 {
                Some(node) => {
                    l2 = node.next;
                    exit = false;
                    node.val
                }
                None => 0,
            };
            if exit && carry == 0 {
                break head;
            }
            *tail = Some(Box::new(ListNode::new(x % 10)));
            tail = &mut tail.as_mut().unwrap().next;
            carry = x / 10;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(vec_to_list(vec![2, 4, 3]), vec_to_list(vec![5, 6, 4])),
            vec_to_list(vec![7, 0, 8])
        );
    }

    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        for &x in nums.iter() {
            *tail = Some(Box::new(ListNode::new(x)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }
}
