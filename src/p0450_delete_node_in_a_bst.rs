#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn find_min(root: Rc<RefCell<TreeNode>>) -> i32 {
        let mut min = root.borrow().val;
        if let Some(left) = root.borrow().left.clone() {
            min = min.min(Solution::find_min(left));
        }
        if let Some(right) = root.borrow().right.clone() {
            min = min.min(Solution::find_min(right));
        }
        min
    }

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let node = root.as_ref().unwrap();
        if key < node.borrow().val {
            let left = node.borrow().left.clone();
            node.borrow_mut().left = Solution::delete_node(left, key);
            return root;
        } else if key > node.borrow().val {
            let right = node.borrow().right.clone();
            node.borrow_mut().right = Solution::delete_node(right, key);
            return root;
        } else {
            if node.borrow().right.is_none() {
                return node.borrow().left.clone();
            } else if node.borrow().left.is_none() {
                return node.borrow().right.clone();
            } else {
                let min = Solution::find_min(node.borrow().right.as_ref().unwrap().clone());
                node.borrow_mut().val = min;
                let right = node.borrow().right.clone();
                node.borrow_mut().right = Solution::delete_node(right, min);
                return root;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_node() {
        assert_eq!(
            Solution::delete_node(tree!(5, 3, 6, 2, 4, null, 7), 3),
            tree!(5, 4, 6, 2, null, null, 7)
        );
        assert_eq!(
            Solution::delete_node(tree!(5, 3, 6, 2, 4, null, 7), 0),
            tree!(5, 3, 6, 2, 4, null, 7)
        );
        assert_eq!(Solution::delete_node(tree!(), 0), tree!());
    }
}
