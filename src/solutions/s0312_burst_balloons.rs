#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn burst(nums: &[i32], l: usize, r: usize, m: &mut HashMap<(usize, usize), i32>) -> i32 {
        if l > r {
            return 0;
        }
        if let Some(&v) = m.get(&(l, r)) {
            return v;
        }
        let mut coins = 0;
        for i in l..=r {
            coins = coins.max(
                Solution::burst(nums, l, i - 1, m)
                    + Solution::burst(nums, i + 1, r, m)
                    + nums[l - 1] * nums[i] * nums[r + 1],
            );
        }
        m.insert((l, r), coins);
        coins
    }

    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut balloons = nums;
        let mut nums = vec![1];
        nums.append(&mut balloons);
        nums.push(1);
        let mut m = HashMap::new();
        Solution::burst(&nums, 1, nums.len() - 2, &mut m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_coins() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);
    }
}
