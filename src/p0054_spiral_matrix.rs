#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = matrix.len() as i32 - 1;
        let mut n = matrix[0].len() as i32;
        let mut ans = Vec::new();
        let (mut x, mut y) = (0, 0);
        while n > 0 {
            for _ in 0..n {
                if !ans.is_empty() {
                    y += 1;
                }
                ans.push(matrix[x][y]);
            }
            n -= 1;
            if m <= 0 {
                break;
            }
            for _ in 0..m {
                x += 1;
                ans.push(matrix[x][y]);
            }
            m -= 1;
            if n <= 0 {
                break;
            }
            for _ in 0..n {
                y -= 1;
                ans.push(matrix[x][y]);
            }
            n -= 1;
            if m <= 0 {
                break;
            }
            for _ in 0..m {
                x -= 1;
                ans.push(matrix[x][y]);
            }
            m -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
