#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return Solution::max_depth(node.borrow().left.clone())
                .max(Solution::max_depth(node.borrow().right.clone()))
                + 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(Solution::max_depth(tree!(3, 9, 20, null, null, 15, 7)), 3);
        assert_eq!(Solution::max_depth(tree!(1, null, 2)), 2);
        assert_eq!(Solution::max_depth(tree!()), 0);
        assert_eq!(Solution::max_depth(tree!(0)), 1);
    }
}
