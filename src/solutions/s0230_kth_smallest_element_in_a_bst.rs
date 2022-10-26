#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn count(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut root = root;
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
            ans += 1;
            root = node.borrow().right.clone();
        }
        ans
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let n = Solution::count(root.as_ref().unwrap().borrow().left.clone());
        if k <= n {
            Solution::kth_smallest(root.as_ref().unwrap().borrow().left.clone(), k)
        } else if k - n == 1 {
            root.unwrap().borrow().val
        } else {
            Solution::kth_smallest(root.as_ref().unwrap().borrow().right.clone(), k - n - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        assert_eq!(Solution::kth_smallest(tree!(3, 1, 4, null, 2), 1), 1);
        assert_eq!(
            Solution::kth_smallest(tree!(5, 3, 6, 2, 4, null, null, 1), 3),
            3
        );
    }
}
