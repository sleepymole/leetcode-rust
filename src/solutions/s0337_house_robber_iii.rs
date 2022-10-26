#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let (mut rob_here, mut not_rob_here) = (root.borrow().val, 0);
        if let Some(ref left) = root.borrow().left {
            let res = Solution::dfs(left.clone());
            rob_here += res.1;
            not_rob_here += res.0.max(res.1);
        }
        if let Some(ref right) = root.borrow().right {
            let res = Solution::dfs(right.clone());
            rob_here += res.1;
            not_rob_here += res.0.max(res.1);
        }
        (rob_here, not_rob_here)
    }

    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let res = Solution::dfs(node);
            res.0.max(res.1)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(tree!(3, 2, 3, null, 3, null, 1)), 7);
        assert_eq!(Solution::rob(tree!(3, 4, 5, 1, 3, null, 1)), 9);
    }
}
