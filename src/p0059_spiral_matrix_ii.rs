#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        matrix.resize(n as usize, vec![]);
        for l in matrix.iter_mut() {
            l.resize(n as usize, 0);
        }
        let mut n = n;
        let mut m = n - 1;
        let mut i = 1;
        let (mut x, mut y) = (0, 0);
        while n > 0 {
            for _ in 0..n {
                if i > 1 {
                    y += 1;
                }
                matrix[x][y] = i;
                i += 1;
            }
            n -= 1;
            if m <= 0 {
                break;
            }
            for _ in 0..m {
                x += 1;
                matrix[x][y] = i;
                i += 1;
            }
            m -= 1;
            if n <= 0 {
                break;
            }
            for _ in 0..n {
                y -= 1;
                matrix[x][y] = i;
                i += 1;
            }
            n -= 1;
            if m <= 0 {
                break;
            }
            for _ in 0..m {
                x -= 1;
                matrix[x][y] = i;
                i += 1;
            }
            m -= 1;
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_matrix() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
