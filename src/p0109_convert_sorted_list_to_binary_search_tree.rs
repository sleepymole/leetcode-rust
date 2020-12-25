#![allow(dead_code)]
pub struct Solution;

use crate::util::{ListNode, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        head.as_ref()?;
        let mut n = 0;
        let mut p = &head;
        while p.is_some() {
            p = &p.as_ref().unwrap().next;
            n += 1;
        }
        let mut head = head;
        let mut mid = &mut head;
        for _ in 0..n / 2 {
            mid = &mut mid.as_mut().unwrap().next;
        }
        let mut root = mid.take();
        let right = root.as_mut().unwrap().next.take();
        Some(Rc::new(RefCell::new(TreeNode {
            val: root.as_ref().unwrap().val,
            left: Solution::sorted_list_to_bst(head),
            right: Solution::sorted_list_to_bst(right),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_list_to_bst() {
        assert_eq!(
            Solution::sorted_list_to_bst(list!(-10, -3, 0, 5, 9)),
            tree!(0, -3, 9, -10, null, 5)
        );
        assert_eq!(Solution::sorted_list_to_bst(list!()), tree!());
        assert_eq!(Solution::sorted_list_to_bst(list!(0)), tree!(0));
        assert_eq!(Solution::sorted_list_to_bst(list!(1, 3)), tree!(3, 1));
    }
}
