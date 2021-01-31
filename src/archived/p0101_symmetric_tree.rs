#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn check(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() || q.is_none() {
            return p.is_none() && q.is_none();
        }
        match (p, q) {
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Solution::check(p.borrow().left.clone(), q.borrow().right.clone())
                    && Solution::check(p.borrow().right.clone(), q.borrow().left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(p) = root {
            return Solution::check(p.borrow().left.clone(), p.borrow().right.clone());
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symmetric() {
        assert_eq!(Solution::is_symmetric(tree!(1, 2, 2, 3, 4, 4, 3)), true);
        assert_eq!(
            Solution::is_symmetric(tree!(1, 2, 2, null, 3, null, 3)),
            false
        );
    }
}
