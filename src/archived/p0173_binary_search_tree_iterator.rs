#![allow(dead_code)]

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator { root }
    }

    fn next(&mut self) -> i32 {
        if self.root.is_none() {
            panic!("end of iterator");
        }
        let mut root = self.root.clone();
        while let Some(node) = root {
            if let Some(mut cur) = node.borrow().left.clone() {
                while cur.borrow().right.is_some()
                    && !Rc::ptr_eq(cur.borrow().right.as_ref().unwrap(), &node)
                {
                    let x = cur.borrow().right.clone().unwrap();
                    cur = x;
                }
                if cur.borrow().right.is_none() {
                    cur.borrow_mut().right = Some(node.clone());
                    root = node.borrow().left.clone();
                    continue;
                }
            }
            self.root = node.borrow().right.clone();
            return node.borrow().val;
        }
        unreachable!();
    }

    fn has_next(&self) -> bool {
        self.root.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_iterator() {
        let mut iter = BSTIterator::new(tree!(7, 3, 15, null, null, 9, 20));
        assert_eq!(iter.next(), 3);
        assert_eq!(iter.next(), 7);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 9);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 15);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 20);
        assert_eq!(iter.has_next(), false);
    }
}
