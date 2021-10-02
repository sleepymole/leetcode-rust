#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::new();
        for x in nums {
            *m.entry(x).or_insert(0) += 1;
        }
        let mut res = 0;
        for (&k, &v) in &m {
            if let Some(&l) = m.get(&(k - 1)) {
                res = res.max(l + v);
            }
            if let Some(&r) = m.get(&(k + 1)) {
                res = res.max(r + v);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lhs() {
        assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
        assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
    }
}
