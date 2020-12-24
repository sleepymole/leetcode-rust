#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root.clone();
        let (mut first, mut second) = (None, None);
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
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
                    continue;
                }
                cur.borrow_mut().right = None;
            }
            if prev
                .as_ref()
                .map_or(false, |x| x.borrow().val > node.borrow().val)
            {
                if first.is_none() {
                    first = prev.clone();
                }
                second = Some(node.clone());
            }
            prev = Some(node.clone());
            root = node.borrow().right.clone();
        }
        if first.is_some() && second.is_some() {
            mem::swap(
                &mut first.as_ref().unwrap().borrow_mut().val,
                &mut second.as_ref().unwrap().borrow_mut().val,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_tree() {
        let mut tree = tree!(1, 3, null, null, 2);
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree!(3, 1, null, null, 2));
        let mut tree = tree!(3, 1, 4, null, null, 2);
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree!(2, 1, 4, null, null, 3));
    }
}
