#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_val = root.as_ref().unwrap().borrow().val;
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        if p_val == root_val
            || q_val == root_val
            || p_val < root_val && q_val > root_val
            || p_val > root_val && q_val < root_val
        {
            root
        } else if p_val < root_val && q_val < root_val {
            Solution::lowest_common_ancestor(root.as_ref().unwrap().borrow().left.clone(), p, q)
        } else {
            Solution::lowest_common_ancestor(root.as_ref().unwrap().borrow().right.clone(), p, q)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree!(6, 2, 8, 0, 4, 7, 9, null, null, 3, 5),
                tree!(2, 0, 4, null, null, 3, 5),
                tree!(8, 7, 8)
            ),
            tree!(6, 2, 8, 0, 4, 7, 9, null, null, 3, 5)
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree!(6, 2, 8, 0, 4, 7, 9, null, null, 3, 5),
                tree!(2, 0, 4, null, null, 3, 5),
                tree!(4, 3, 5)
            ),
            tree!(2, 0, 4, null, null, 3, 5)
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree!(2, 1, null),
                tree!(2, 1, null),
                tree!(1, null, null)
            ),
            tree!(2, 1, null)
        );
    }
}
