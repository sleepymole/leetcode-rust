#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn traverse<F>(mut root: Option<Rc<RefCell<TreeNode>>>, mut f: F)
    where
        F: FnMut(i32),
    {
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
                cur.borrow_mut().right = None;
            }
            f(node.borrow().val);
            root = node.borrow().right.clone();
        }
    }

    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut freq = 0;
        let mut max_freq = 0;
        let mut prev = i32::MIN;
        Solution::traverse(root.clone(), |val: i32| {
            if val == prev {
                freq += 1;
            } else {
                prev = val;
                freq = 1;
            }
            max_freq = max_freq.max(freq);
        });
        let mut res = Vec::new();
        freq = 0;
        prev = i32::MIN;
        Solution::traverse(root, |val: i32| {
            if val == prev {
                freq += 1;
            } else {
                prev = val;
                freq = 1;
            }
            if freq == max_freq {
                res.push(val);
            }
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mode() {
        assert_eq!(Solution::find_mode(tree!(1, null, 2, 2)), vec![2]);
        assert_eq!(Solution::find_mode(tree!(0)), vec![0]);
    }
}
