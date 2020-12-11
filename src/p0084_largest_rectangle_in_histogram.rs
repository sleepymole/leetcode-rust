#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut left = Vec::new();
        let mut right = Vec::new();
        left.resize(heights.len(), -1);
        right.resize(heights.len(), heights.len() as i32);
        let mut stk = vec![-1];
        for i in 0..heights.len() {
            while stk.len() > 1 && heights[stk.last().copied().unwrap() as usize] >= heights[i] {
                stk.pop();
            }
            left[i] = stk.last().copied().unwrap();
            stk.push(i as i32);
        }
        let mut stk = vec![heights.len() as i32];
        for i in (0..heights.len()).rev() {
            while stk.len() > 1 && heights[stk.last().copied().unwrap() as usize] >= heights[i] {
                stk.pop();
            }
            right[i] = stk.last().copied().unwrap();
            stk.push(i as i32);
        }
        let mut ans = 0;
        for i in 0..heights.len() {
            ans = ans.max((right[i] - left[i] - 1) * heights[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
