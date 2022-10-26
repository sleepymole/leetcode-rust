#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() || q.is_none() {
            return p.is_none() && q.is_none();
        }
        if p.as_ref().unwrap().borrow().val != q.as_ref().unwrap().borrow().val {
            return false;
        }
        Solution::is_same_tree(
            p.as_ref().unwrap().borrow().left.clone(),
            q.as_ref().unwrap().borrow().left.clone(),
        ) && Solution::is_same_tree(
            p.as_ref().unwrap().borrow().right.clone(),
            q.as_ref().unwrap().borrow().right.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        assert!(Solution::is_same_tree(tree!(1, 2, 3), tree!(1, 2, 3)));
        assert!(!Solution::is_same_tree(tree!(1, 2), tree!(1, null, 2)));
        assert!(!Solution::is_same_tree(tree!(1, 2, 1), tree!(1, 1, 2)));
    }
}
