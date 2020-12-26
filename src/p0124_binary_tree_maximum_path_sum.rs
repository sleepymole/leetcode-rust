#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::i32;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        let rc_root = root.as_ref().unwrap().borrow();
        let root_val = rc_root.val;
        let left = rc_root.left.as_ref();
        let right = rc_root.right.as_ref();
        let mut sum = root_val;
        let mut max_sum = sum;
        let mut max_root_sum = root_val;
        if let Some(node) = left {
            let (l_max_sum, l_max_root_sum) = Solution::dfs(Some(node));
            if l_max_root_sum > 0 {
                sum += l_max_root_sum;
                max_root_sum = max_root_sum.max(root_val + l_max_root_sum);
            }
            max_sum = max_sum.max(sum).max(l_max_sum);
        }
        if let Some(node) = right {
            let (r_max_sum, r_max_root_sum) = Solution::dfs(Some(node));
            if r_max_root_sum > 0 {
                sum += r_max_root_sum;
                max_root_sum = max_root_sum.max(root_val + r_max_root_sum);
            }
            max_sum = max_sum.max(sum).max(r_max_sum);
        }
        (max_sum, max_root_sum)
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let (ans, _) = Solution::dfs(root.as_ref());
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_path_sum() {
        assert_eq!(Solution::max_path_sum(tree!(1, 2, 3)), 6);
        assert_eq!(
            Solution::max_path_sum(tree!(-10, 9, 20, null, null, 15, 7)),
            42
        );
        assert_eq!(Solution::max_path_sum(tree!(1, -2, 3)), 4);
    }
}
