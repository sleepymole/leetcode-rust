#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        if let Some(pos) = inorder
            .iter()
            .position(|&x| x == postorder[postorder.len() - 1])
        {
            Some(Rc::new(RefCell::new(TreeNode {
                val: postorder[postorder.len() - 1],
                left: Solution::build_tree(inorder[0..pos].to_vec(), postorder[0..pos].to_vec()),
                right: Solution::build_tree(
                    inorder[pos + 1..].to_vec(),
                    postorder[pos..inorder.len() - 1].to_vec(),
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
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            tree!(3, 9, 20, null, null, 15, 7)
        );
    }
}
