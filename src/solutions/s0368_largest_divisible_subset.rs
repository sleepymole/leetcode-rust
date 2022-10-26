#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut f: Vec<i32> = vec![1; nums.len()];
        let mut prev = vec![None; nums.len()];
        let mut target = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && f[j] + 1 > f[i] {
                    f[i] = f[j] + 1;
                    prev[i] = Some(j);
                    if f[i] > f[target] {
                        target = i;
                    }
                }
            }
        }
        let mut set = vec![nums[target]];
        while let Some(p) = prev[target] {
            set.push(nums[p]);
            target = p;
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate(mut nums: Vec<i32>) {
        nums.sort_unstable();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                assert!(nums[j] > nums[i] && nums[j] % nums[i] == 0);
            }
        }
    }

    #[test]
    fn test_largest_divisible_subset() {
        let nums = Solution::largest_divisible_subset(vec![1, 2, 3]);
        assert_eq!(nums.len(), 2);
        validate(nums);
        let nums = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
        assert_eq!(nums.len(), 4);
        validate(nums);
    }
}
