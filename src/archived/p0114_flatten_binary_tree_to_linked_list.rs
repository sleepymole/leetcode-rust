#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root.clone();
        while let Some(node) = root {
            if let Some(mut cur) = node.borrow().left.clone() {
                while cur.borrow().right.is_some() {
                    let right = cur.borrow().right.clone().unwrap();
                    cur = right;
                }
                cur.borrow_mut().right = node.borrow().right.clone();
                root = node.borrow().left.clone();
            } else {
                root = node.borrow().right.clone();
            }
            node.borrow_mut().left = None;
            node.borrow_mut().right = root.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten() {
        let mut root = tree!(1, 2, 5, 3, 4, null, 6);
        Solution::flatten(&mut root);
        assert_eq!(root, tree!(1, null, 2, null, 3, null, 4, null, 5, null, 6));
    }
}
