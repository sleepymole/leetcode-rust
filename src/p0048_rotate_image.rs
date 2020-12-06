#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..(n + 1) / 2 {
            for j in 0..n / 2 {
                let t = matrix[n - j - 1][i];
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = matrix[i][j];
                matrix[i][j] = t;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
        let mut m = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut m);
        assert_eq!(
            m,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
        let mut m = vec![vec![1]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![1]]);
        let mut m = vec![vec![1, 2], vec![3, 4]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![3, 1], vec![4, 2]]);
    }
}
