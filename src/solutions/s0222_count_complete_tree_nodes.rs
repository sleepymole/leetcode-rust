#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut left_high = 0;
            let mut cur = Some(node.clone());
            while let Some(node) = cur {
                cur = node.borrow().left.clone();
                left_high += 1;
            }
            let mut right_high = 0;
            let mut cur = Some(node.clone());
            while let Some(node) = cur {
                cur = node.borrow().right.clone();
                right_high += 1;
            }
            if left_high == right_high {
                return (1 << left_high) - 1;
            }
            return 1
                + Solution::count_nodes(node.borrow().left.clone())
                + Solution::count_nodes(node.borrow().right.clone());
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nodes() {
        assert_eq!(Solution::count_nodes(tree!(1, 2, 3, 4, 5, 6)), 6);
        assert_eq!(Solution::count_nodes(tree!()), 0);
        assert_eq!(Solution::count_nodes(tree!(1)), 1);
    }
}
