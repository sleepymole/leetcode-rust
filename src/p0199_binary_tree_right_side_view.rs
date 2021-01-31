#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        }
        let mut ans = Vec::new();
        while !q.is_empty() {
            let mut right_most = 0;
            let mut q2 = VecDeque::new();
            while let Some(node) = q.pop_front() {
                if let Some(left) = &node.borrow().left {
                    q2.push_back(left.clone());
                }
                if let Some(right) = &node.borrow().right {
                    q2.push_back(right.clone());
                }
                right_most = node.borrow().val;
            }
            q = q2;
            ans.push(right_most);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_side_view() {
        assert_eq!(
            Solution::right_side_view(tree!(1, 2, 3, null, 5, null, 4)),
            vec![1, 3, 4]
        );
    }
}
