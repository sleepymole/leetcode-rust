#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let (mut current, mut farthest) = (0, 0);
        for i in 0..nums.len() - 1 {
            farthest = i32::max(farthest, i as i32 + nums[i]);
            if i as i32 == current {
                steps += 1;
                current = farthest;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jump() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
