#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let (mut l, mut r) = (*nums.iter().max().unwrap(), nums.iter().sum());
        while l < r {
            let mid = (l + r) / 2;
            let mut count = 0;
            let mut i = 0;
            while i < nums.len() {
                let mut sum = 0;
                let mut j = i;
                while j < nums.len() && sum + nums[j] <= mid {
                    sum += nums[j];
                    j += 1;
                }
                count += 1;
                i = j;
            }
            if count <= m {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_array() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
    }
}
