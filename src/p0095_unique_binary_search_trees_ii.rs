#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn make_trees(n: i32, extra: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![None];
        }
        let mut result = Vec::new();
        for i in 1..=n {
            let left_trees = Solution::make_trees(i - 1, extra);
            let right_trees = Solution::make_trees(n - i, extra + i);
            for l in left_trees.iter() {
                for r in right_trees.iter() {
                    let root = Rc::new(RefCell::new(TreeNode::new(i + extra)));
                    root.borrow_mut().left = l.clone();
                    root.borrow_mut().right = r.clone();
                    result.push(Some(root));
                }
            }
        }
        result
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        Solution::make_trees(n, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_trees() {
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                tree!(1, null, 2, null, 3),
                tree!(1, null, 3, 2),
                tree!(2, 1, 3),
                tree!(3, 1, null, null, 2),
                tree!(3, 2, null, 1)
            ]
        );
    }
}
