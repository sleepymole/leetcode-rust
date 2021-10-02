#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut root = root;
        let mut prev = -200000;
        let mut min_diff = i32::MAX;
        while let Some(node) = root {
            if let Some(mut cur) = node.borrow().left.clone() {
                while cur.borrow().right.is_some()
                    && !Rc::ptr_eq(cur.borrow().right.as_ref().unwrap(), &node)
                {
                    let x = cur.borrow().right.clone().unwrap();
                    cur = x;
                }
                if cur.borrow().right.is_none() {
                    cur.borrow_mut().right = Some(node.clone());
                    root = node.borrow().left.clone();
                    continue;
                }
                cur.borrow_mut().right = None;
            }
            let val = node.borrow().val;
            min_diff = min_diff.min(val - prev);
            prev = val;
            root = node.borrow().right.clone();
        }
        min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_minimum_difference() {
        assert_eq!(Solution::get_minimum_difference(tree!(4, 2, 6, 1, 3)), 1);
        assert_eq!(
            Solution::get_minimum_difference(tree!(1, 0, 48, null, null, 12, 49)),
            1
        );
    }
}
