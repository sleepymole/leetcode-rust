#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut cnts = Vec::new();
        cnts.resize(3, 0);
        for x in nums.iter() {
            cnts[*x as usize] += 1
        }
        let mut i = 0;
        for j in 0..3 {
            while cnts[j] > 0 {
                nums[i] = j as i32;
                i += 1;
                cnts[j] -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
        let mut nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);
        let mut nums = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}
