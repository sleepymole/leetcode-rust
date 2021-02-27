#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut level = 0;
        while (1 << level) < nums.len() {
            level += 1;
        }
        let mut max = vec![vec![0; nums.len()]; level + 1];
        max[0] = nums.clone();
        for j in 1..=level {
            let mut i = 0;
            while i + (1 << j) <= nums.len() {
                max[j][i] = max[j - 1][i].max(max[j - 1][i + (1 << (j - 1))]);
                i += 1;
            }
        }
        let mut i = 0;
        let k = k as usize;
        let mut ans = Vec::new();
        while i + k <= nums.len() {
            let mut m = nums[i];
            let mut j = i;
            while j < i + k {
                let mut l = 0;
                while j + (1 << (l + 1)) <= i + k {
                    l += 1;
                }
                m = m.max(max[l][j]);
                j += 1 << l;
            }
            ans.push(m);
            i += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
        assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(Solution::max_sliding_window(vec![9, 11], 2), vec![11]);
        assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]);
    }
}
