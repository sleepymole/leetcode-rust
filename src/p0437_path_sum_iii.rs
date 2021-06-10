#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut sum: i32,
        prefix: &mut HashMap<i32, i32>,
        target: i32,
    ) -> i32 {
        if let Some(node) = root {
            let mut count = 0;
            let val = node.borrow().val;
            sum += val;
            if let Some(&v) = prefix.get(&(sum - target)) {
                count += v;
            }
            *prefix.entry(sum).or_insert(0) += 1;
            count += Solution::dfs(node.borrow().left.clone(), sum, prefix, target);
            count += Solution::dfs(node.borrow().right.clone(), sum, prefix, target);
            if let Some(v) = prefix.get_mut(&sum) {
                *v -= 1;
                if *v == 0 {
                    prefix.remove(&sum);
                }
            }
            count
        } else {
            0
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut prefix = HashMap::new();
        prefix.insert(0, 1);
        Solution::dfs(root, 0, &mut prefix, target_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_sum() {
        assert_eq!(
            Solution::path_sum(tree!(10, 5, -3, 3, 2, null, 11, 3, -2, null, 1), 8),
            3
        );
        assert_eq!(
            Solution::path_sum(tree!(5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1), 22),
            3
        );
    }
}
