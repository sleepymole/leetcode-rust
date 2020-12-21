#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut first, mut second) = (None, None);
        let mut cur = root.clone();
        let mut tmp: Option<Rc<RefCell<TreeNode>>> = None;
        while cur.is_some() {
            if cur.as_ref().unwrap().borrow().left.is_some() {
                let mut prev = cur.as_ref().unwrap().borrow().left.clone();
                while prev.as_ref().unwrap().borrow().right.is_some()
                    && prev
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .val
                        != cur.as_ref().unwrap().borrow().val
                {
                    prev = prev.unwrap().borrow().right.clone();
                }
                if prev.as_ref().unwrap().borrow().right.is_none() {
                    prev.as_ref().unwrap().borrow_mut().right = cur.clone();
                    cur = cur.unwrap().borrow().left.clone();
                    continue;
                }
                prev.as_ref().unwrap().borrow_mut().right = None;
            }
            if tmp.is_some()
                && tmp.as_ref().unwrap().borrow().val > cur.as_ref().unwrap().borrow().val
            {
                if first.is_none() {
                    first = tmp.clone();
                    second = cur.clone();
                } else {
                    second = cur.clone();
                }
            }
            tmp = cur.clone();
            cur = cur.unwrap().borrow().right.clone();
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
