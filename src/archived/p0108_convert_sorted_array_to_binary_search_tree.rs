#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Solution::sorted_array_to_bst(nums[0..mid].to_vec()),
            right: Solution::sorted_array_to_bst(nums[mid + 1..].to_vec()),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array_to_bst() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            tree!(0, -3, 9, -10, null, 5)
        )
    }
}
