#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut orders = Vec::new();
        for i in 0..=(m + n - 2) {
            if i % 2 == 0 {
                let mut x = i.min(m - 1);
                let mut y = i - x;
                while {
                    orders.push(mat[x][y]);
                    x > 0 && y + 1 < n
                } {
                    x -= 1;
                    y += 1;
                }
            } else {
                let mut y = i.min(n - 1);
                let mut x = i - y;
                while {
                    orders.push(mat[x][y]);
                    x + 1 < m && y > 0
                } {
                    x += 1;
                    y -= 1;
                }
            }
        }
        orders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_diagonal_order() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 3, 4]
        );
    }
}
