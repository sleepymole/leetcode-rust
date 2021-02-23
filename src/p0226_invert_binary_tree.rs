#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref node) = root {
            let left = Solution::invert_tree(node.borrow().left.clone());
            let right = Solution::invert_tree(node.borrow().right.clone());
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_tree() {
        assert_eq!(
            Solution::invert_tree(tree!(4, 2, 7, 1, 3, 6, 9)),
            tree!(4, 7, 2, 9, 6, 3, 1)
        );
    }
}
