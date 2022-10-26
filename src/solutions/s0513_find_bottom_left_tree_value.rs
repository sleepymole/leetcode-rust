#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut left_most = 0;
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        }
        while !q.is_empty() {
            let n = q.len();
            left_most = q.front().unwrap().borrow().val;
            for _ in 0..n {
                if let Some(node) = q.pop_front() {
                    if let Some(left) = node.borrow().left.clone() {
                        q.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        q.push_back(right);
                    }
                }
            }
        }
        left_most
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bottom_left_value() {
        assert_eq!(Solution::find_bottom_left_value(tree!(2, 1, 3)), 1);
        assert_eq!(
            Solution::find_bottom_left_value(tree!(1, 2, 3, 4, null, 5, 6, null, null, 7)),
            7
        );
    }
}
