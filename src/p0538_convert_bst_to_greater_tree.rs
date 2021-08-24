#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Rc<RefCell<TreeNode>>, extra: i32) -> i32 {
        let mut sum = root.borrow().val;
        if let Some(right) = &root.borrow().right {
            sum += Solution::dfs(right.clone(), extra);
        }
        root.borrow_mut().val = extra + sum;
        if let Some(left) = &root.borrow().left {
            sum += Solution::dfs(left.clone(), extra + sum);
        }
        sum
    }

    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            Solution::dfs(root.clone(), 0);
            Some(root)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_bst() {
        assert_eq!(
            Solution::convert_bst(tree!(
                4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
            )),
            tree!(30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8)
        );
        assert_eq!(Solution::convert_bst(tree!(0, null, 1)), tree!(1, null, 1));
        assert_eq!(Solution::convert_bst(tree!(1, 0, 2)), tree!(3, 3, 2));
        assert_eq!(Solution::convert_bst(tree!(3, 2, 4, 1)), tree!(7, 9, 4, 10));
    }
}
