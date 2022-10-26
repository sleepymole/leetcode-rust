#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut s = String::new();
        Solution::traverse(root, &mut s);
        s
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, s: &mut String) {
        if let Some(node) = root {
            let node = node.borrow();
            s.push_str(&node.val.to_string());
            if node.left.is_some() || node.right.is_some() {
                s.push('(');
                Self::traverse(node.left.clone(), s);
                s.push(')');
                if node.right.is_some() {
                    s.push('(');
                    Self::traverse(node.right.clone(), s);
                    s.push(')');
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree2str() {
        assert_eq!(
            Solution::tree2str(tree!(1, 2, 3, 4)),
            "1(2(4))(3)".to_owned()
        );
        assert_eq!(
            Solution::tree2str(tree!(1, 2, 3, null, 4)),
            "1(2()(4))(3)".to_owned()
        );
    }
}
