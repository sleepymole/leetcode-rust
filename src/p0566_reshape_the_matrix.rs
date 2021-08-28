#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let (r, c) = (r as usize, c as usize);
        if m * n != r * c {
            return mat;
        }
        let mut new_mat = vec![vec![0; c]; r];
        for i in 0..(m * n) {
            new_mat[i / c][i % c] = mat[i / n][i % n];
        }
        new_mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_reshape() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 2),
            vec![vec![1, 2], vec![3, 4]]
        );
    }
}
