#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        }
        while !q.is_empty() {
            let n = q.len();
            let mut max = q.front().unwrap().borrow().val;
            for _ in 0..n {
                if let Some(node) = q.pop_front() {
                    max = max.max(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        q.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        q.push_back(right);
                    }
                }
            }
            res.push(max);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_values() {
        assert_eq!(
            Solution::largest_values(tree!(1, 3, 2, 5, 3, null, 9)),
            vec![1, 3, 9]
        );
        assert_eq!(Solution::largest_values(tree!(1, 2, 3)), vec![1, 3]);
        assert_eq!(Solution::largest_values(tree!(1)), vec![1]);
        assert_eq!(Solution::largest_values(tree!(1, null, 2)), vec![1, 2]);
        assert!(Solution::largest_values(tree!()).is_empty());
    }
}
