#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Rc<RefCell<TreeNode>>, diameter: &mut i32) -> i32 {
        let mut left_depth = 0;
        let mut right_depth = 0;
        if let Some(left) = &root.borrow().left {
            left_depth = Solution::dfs(left.clone(), diameter) + 1;
        }
        if let Some(right) = &root.borrow().right {
            right_depth = Solution::dfs(right.clone(), diameter) + 1;
        }
        *diameter = (*diameter).max(left_depth + right_depth);
        left_depth.max(right_depth)
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        if let Some(root) = root {
            Solution::dfs(root, &mut diameter);
        }
        diameter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diameter_of_binary_tree() {
        assert_eq!(Solution::diameter_of_binary_tree(tree!(1, 2, 3, 4, 5)), 3);
        assert_eq!(Solution::diameter_of_binary_tree(tree!(1, 2)), 1);
    }
}
