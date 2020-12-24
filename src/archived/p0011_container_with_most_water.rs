#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut m = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            m = std::cmp::max(m, std::cmp::min(height[l], height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
