#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stk = vec![root.unwrap()];
        let sub_root = sub_root.unwrap();
        while let Some(root) = stk.pop() {
            if *root.borrow() == *sub_root.borrow() {
                return true;
            }
            if let Some(left) = &root.borrow().left {
                stk.push(left.clone());
            }
            if let Some(right) = &root.borrow().right {
                stk.push(right.clone());
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subtree() {
        assert!(Solution::is_subtree(tree!(3, 4, 5, 1, 2), tree!(4, 1, 2)));
        assert!(!Solution::is_subtree(
            tree!(3, 4, 5, 1, 2, null, null, null, null, 0),
            tree!(4, 1, 2)
        ));
    }
}
