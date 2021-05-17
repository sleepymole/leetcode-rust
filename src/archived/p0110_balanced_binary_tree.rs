#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> (usize, bool) {
        if root.is_none() {
            return (0, true);
        }
        let (l_len, l_ok) = Solution::dfs(root.unwrap().borrow().left.as_ref());
        let (r_len, r_ok) = Solution::dfs(root.unwrap().borrow().right.as_ref());
        (
            l_len.max(r_len) + 1,
            l_ok && r_ok && l_len.max(r_len) - l_len.min(r_len) <= 1,
        )
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (_, ok) = Solution::dfs(root.as_ref());
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced() {
        assert!(
            Solution::is_balanced(tree!(3, 9, 20, null, null, 15, 7)));
        assert!(!
            Solution::is_balanced(tree!(1, 2, 2, 3, 3, null, null, 4, 4)));
        assert!(Solution::is_balanced(tree!()));
    }
}
