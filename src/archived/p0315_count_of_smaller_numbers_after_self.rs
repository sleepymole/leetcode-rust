#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut rank = nums.clone();
        rank.sort_unstable();
        let mut nums = nums;
        for i in 0..nums.len() {
            nums[i] = rank.binary_search(&nums[i]).unwrap() as i32 + 1;
        }
        let mut sums = vec![0; nums.len() + 1];
        let mut counts = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            let mut x = nums[i] - 1;
            while x > 0 {
                counts[i] += sums[x as usize];
                x -= x & (-x);
            }
            let mut x = nums[i];
            while x <= nums.len() as i32 {
                sums[x as usize] += 1;
                x += x & (-x);
            }
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_smaller() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }
}
