#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        let sum = sum - root.as_ref().unwrap().borrow().val;
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        if left.is_none() && right.is_none() {
            return sum == 0;
        }
        if left.is_some() && Solution::dfs(left, sum) {
            return true;
        }
        if right.is_some() && Solution::dfs(right, sum) {
            return true;
        }
        false
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        Solution::dfs(root, sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_path_sum() {
        assert!(Solution::has_path_sum(
            tree!(5, 4, 8, 11, null, 13, 4, 7, 2, null, 1),
            22
        ));
    }
}
