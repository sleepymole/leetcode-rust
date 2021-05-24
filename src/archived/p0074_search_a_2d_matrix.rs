#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        if m == 0 {
            return false;
        }
        let n = matrix[0].len();
        let (mut l, mut r) = (0, m * n);
        while l < r {
            let mid = (l + r) / 2;
            if matrix[mid / n][mid % n] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if l == m * n {
            return false;
        }
        matrix[l / n][l % n] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            3
        ));
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            13
        ));
    }
}
