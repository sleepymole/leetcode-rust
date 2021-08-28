#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root) = root {
            let (lsum, ltilt) = Solution::dfs(root.borrow().left.clone());
            let (rsum, rtilt) = Solution::dfs(root.borrow().right.clone());
            (
                root.borrow().val + lsum + rsum,
                ltilt + rtilt + (lsum - rsum).abs(),
            )
        } else {
            (0, 0)
        }
    }

    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_tilt() {
        assert_eq!(Solution::find_tilt(tree!(1, 2, 3)), 1);
        assert_eq!(Solution::find_tilt(tree!(4, 2, 9, 3, 5, null, 7)), 15);
        assert_eq!(Solution::find_tilt(tree!(21, 7, 14, 1, 1, 2, 2, 3, 3)), 9);
    }
}
