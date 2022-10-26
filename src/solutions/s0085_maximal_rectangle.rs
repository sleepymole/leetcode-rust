#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
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

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut h: Vec<Vec<i32>> = Vec::new();
        h.resize(m, vec![]);
        for l in h.iter_mut() {
            l.resize(n, 0);
        }
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    h[i][j] = 1;
                    if i > 0 && matrix[i - 1][j] == '1' {
                        h[i][j] += h[i - 1][j];
                    }
                }
            }
        }
        let mut ans = 0;
        for l in h.iter() {
            ans = ans.max(Solution::largest_rectangle_area(l));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_rectangle() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
        assert_eq!(Solution::maximal_rectangle(vec![vec![]]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0', '0']]), 0);
        assert_eq!(
            Solution::maximal_rectangle(vec![vec!['0', '1'], vec!['1', '0']]),
            1
        );
    }
}
