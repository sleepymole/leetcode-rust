#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn traverse(parent: Rc<RefCell<TreeNode>>) -> Option<(i32, i32)> {
        let mut min_val = parent.borrow().val;
        let mut max_val = parent.borrow().val;
        if let Some(ref v) = parent.borrow().left {
            if let Some((mi, ma)) = Solution::traverse(v.clone()) {
                if ma >= parent.borrow().val {
                    return None;
                }
                min_val = min_val.min(mi);
                max_val = max_val.max(ma);
            } else {
                return None;
            }
        }
        if let Some(ref v) = parent.borrow().right {
            if let Some((mi, ma)) = Solution::traverse(v.clone()) {
                if mi <= parent.borrow().val {
                    return None;
                }
                min_val = min_val.min(mi);
                max_val = max_val.max(ma);
            } else {
                return None;
            }
        }
        Some((min_val, max_val))
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        Solution::traverse(root.unwrap()).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_bst() {
        assert!(Solution::is_valid_bst(tree!(2, 1, 3)));
        assert!(!
            Solution::is_valid_bst(tree!(5, 1, 4, null, null, 3, 6)));
        assert!(!Solution::is_valid_bst(tree!(5, 4, 6, null, null, 3)));
    }
}
