#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut vis = vec![false; nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            if vis[i] {
                continue;
            }
            let mut x = i;
            let mut len = 1;
            while nums[x] as usize != i {
                x = nums[x] as usize;
                len += 1;
                vis[x] = true;
            }
            res = res.max(len);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_nesting() {
        assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
        assert_eq!(Solution::array_nesting(vec![0, 1, 2]), 1);
    }
}
