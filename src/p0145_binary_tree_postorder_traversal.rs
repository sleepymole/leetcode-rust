#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            left: root,
            right: None,
            val: 0,
        })));
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
                } else {
                    cur.borrow_mut().right = None;
                    root = node.borrow().right.clone();
                    let mut i = ans.len();
                    let mut iter = node.borrow().left.clone();
                    while let Some(cur) = iter {
                        ans.push(cur.borrow().val);
                        iter = cur.borrow().right.clone();
                    }
                    let mut j = ans.len() - 1;
                    while i < j {
                        ans.swap(i, j);
                        i += 1;
                        j -= 1;
                    }
                }
            } else {
                root = node.borrow().right.clone();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postorder_traversal() {
        assert_eq!(
            Solution::postorder_traversal(tree!(1, null, 2, 3)),
            vec![3, 2, 1]
        );
        assert_eq!(Solution::postorder_traversal(tree!()), Vec::<i32>::new());
        assert_eq!(Solution::postorder_traversal(tree!(1)), vec![1]);
        assert_eq!(Solution::postorder_traversal(tree!(1, 2)), vec![2, 1]);
        assert_eq!(Solution::postorder_traversal(tree!(1, null, 2)), vec![2, 1]);
    }
}
