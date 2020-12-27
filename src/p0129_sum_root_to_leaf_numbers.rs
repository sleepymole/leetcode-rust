#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32, total_sum: &mut i32) {
        let rc_root = root.unwrap().borrow();
        let sum = sum * 10 + rc_root.val;
        if rc_root.left.is_none() && rc_root.right.is_none() {
            *total_sum += sum;
            return;
        }
        if rc_root.left.is_some() {
            Solution::dfs(rc_root.left.as_ref(), sum, total_sum);
        }
        if rc_root.right.is_some() {
            Solution::dfs(rc_root.right.as_ref(), sum, total_sum);
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut total_sum = 0;
        Solution::dfs(root.as_ref(), 0, &mut total_sum);
        total_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers() {
        assert_eq!(Solution::sum_numbers(tree!(1, 2, 3)), 25);
        assert_eq!(Solution::sum_numbers(tree!(4, 9, 0, 5, 1)), 1026);
    }
}
