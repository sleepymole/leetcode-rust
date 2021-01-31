#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let l = Solution::min_depth(root.as_ref().unwrap().borrow().left.clone());
        let r = Solution::min_depth(root.as_ref().unwrap().borrow().right.clone());
        match (l > 0, r > 0) {
            (true, true) => l.min(r) + 1,
            (true, false) => l + 1,
            (false, true) => r + 1,
            (false, false) => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_depth() {
        assert_eq!(Solution::min_depth(tree!(3, 9, 20, null, null, 15, 7)), 2);
        assert_eq!(
            Solution::min_depth(tree!(2, null, 3, null, 4, null, 5, null, 6)),
            5
        );
    }
}
