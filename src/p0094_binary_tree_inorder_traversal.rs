#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut nums = Vec::new();
        let mut root = root;
        while let Some(node) = root {
            if let Some(mut cur) = node.borrow().left.clone() {
                while cur.borrow().right.is_some()
                    && !Rc::ptr_eq(cur.borrow().right.as_ref().unwrap(), &node)
                {
                    let right = cur.borrow().right.clone().unwrap();
                    cur = right;
                }
                if cur.borrow().right.is_none() {
                    cur.borrow_mut().right = Some(node.clone());
                    root = node.borrow().left.clone();
                    continue;
                }
                cur.borrow_mut().right = None;
            }
            nums.push(node.borrow().val);
            root = node.borrow().right.clone();
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
