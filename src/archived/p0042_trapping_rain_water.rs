#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (Vec::new(), Vec::new());
        left.resize(height.len(), 0);
        right.resize(height.len(), 0);
        for i in 0..height.len() {
            if i == 0 || height[i] > left[i - 1] {
                left[i] = height[i];
            } else {
                left[i] = left[i - 1];
            }
        }
        for i in (0..height.len()).rev() {
            if i + 1 == height.len() || height[i] > right[i + 1] {
                right[i] = height[i];
            } else {
                right[i] = right[i + 1];
            }
        }
        let mut ans = 0;
        for i in 0..height.len() {
            ans += i32::min(left[i], right[i]) - height[i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
