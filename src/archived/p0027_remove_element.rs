#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut n = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[n] = nums[i];
                n += 1;
            }
        }
        nums.truncate(n);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let n = Solution::remove_element(&mut nums, 3);
        assert_eq!(n, 2);
        assert_eq!(nums, vec![2, 2]);
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let n = Solution::remove_element(&mut nums, 2);
        assert_eq!(n, 5);
        nums.sort_unstable();
        assert_eq!(nums, vec![0, 0, 1, 3, 4]);
    }
}
