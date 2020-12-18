#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn traverse(parent: Rc<RefCell<TreeNode>>, nums: &mut Vec<i32>) {
        if let Some(ref v) = parent.borrow().left {
            Solution::traverse(v.clone(), nums);
        }
        nums.push(parent.borrow().val);
        if let Some(ref v) = parent.borrow().right {
            Solution::traverse(v.clone(), nums);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut nums = Vec::new();
        if let Some(v) = root {
            Solution::traverse(v, &mut nums);
        }
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        assert_eq!(
            Solution::inorder_traversal(tree!(1, null, 2, 3)),
            vec![1, 3, 2]
        );
        assert_eq!(Solution::inorder_traversal(tree!()), vec![]);
        assert_eq!(Solution::inorder_traversal(tree!(1)), vec![1]);
        assert_eq!(Solution::inorder_traversal(tree!(1, 2)), vec!(2, 1));
        assert_eq!(Solution::inorder_traversal(tree!(1, null, 2)), vec![1, 2]);
    }
}
