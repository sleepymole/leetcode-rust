#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut orders = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(root.as_ref().unwrap().clone());
        while !q.is_empty() {
            let mut order = Vec::new();
            let n = q.len();
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                order.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    q.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if node.borrow().right.is_some() {
                    q.push_back(node.borrow().right.as_ref().unwrap().clone());
                }
            }
            orders.push(order);
        }
        orders.reverse();
        orders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order() {
        assert_eq!(
            Solution::level_order_bottom(tree!(3, 9, 20, null, null, 15, 7)),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );
    }
}
