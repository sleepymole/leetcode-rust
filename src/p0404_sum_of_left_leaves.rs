#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut root = root;
        while let Some(node) = root {
            if let Some(mut cur) = node.borrow().left.clone() {
                if cur.borrow().left.is_none() && cur.borrow().right.is_none() {
                    sum += cur.borrow().val;
                }
                while cur.borrow().right.is_some()
                    && !Rc::ptr_eq(cur.borrow().right.as_ref().unwrap(), &node)
                {
                    let right = cur.borrow().right.clone();
                    cur = right.unwrap();
                }
                if cur.borrow().right.is_none() {
                    cur.borrow_mut().right = Some(node.clone());
                    root = node.borrow().left.clone();
                    continue;
                }
                cur.borrow_mut().right = None;
            }
            root = node.borrow().right.clone();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_left_leaves() {
        assert_eq!(
            Solution::sum_of_left_leaves(tree!(3, 9, 20, null, null, 15, 7)),
            24
        );
        assert_eq!(Solution::sum_of_left_leaves(tree!(1)), 0);
        assert_eq!(Solution::sum_of_left_leaves(tree!(1, 2, 3, 4, 5)), 4);
    }
}
