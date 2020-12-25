#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        if let Some(pos) = inorder.iter().position(|&x| x == preorder[0]) {
            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[0],
                left: Solution::build_tree(preorder[1..=pos].to_vec(), inorder[0..pos].to_vec()),
                right: Solution::build_tree(
                    preorder[pos + 1..].to_vec(),
                    inorder[pos + 1..].to_vec(),
                ),
            })))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree!(3, 9, 20, null, null, 15, 7)
        );
    }
}
