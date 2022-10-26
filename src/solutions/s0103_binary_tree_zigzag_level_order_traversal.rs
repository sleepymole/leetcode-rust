#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut orders = Vec::new();
        let mut stk = vec![root.as_ref().unwrap().clone()];
        let mut to_right = true;
        while !stk.is_empty() {
            let mut order = Vec::new();
            let mut stk2 = Vec::new();
            while !stk.is_empty() {
                let node = stk.pop().unwrap();
                order.push(node.borrow().val);
                if to_right {
                    if node.borrow().left.is_some() {
                        stk2.push(node.borrow().left.as_ref().unwrap().clone());
                    }
                    if node.borrow().right.is_some() {
                        stk2.push(node.borrow().right.as_ref().unwrap().clone());
                    }
                } else {
                    if node.borrow().right.is_some() {
                        stk2.push(node.borrow().right.as_ref().unwrap().clone());
                    }
                    if node.borrow().left.is_some() {
                        stk2.push(node.borrow().left.as_ref().unwrap().clone());
                    }
                }
            }
            orders.push(order);
            to_right = !to_right;
            stk = stk2;
        }
        orders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_level_order() {
        assert_eq!(
            Solution::zigzag_level_order(tree!(3, 9, 20, null, null, 15, 7)),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}
