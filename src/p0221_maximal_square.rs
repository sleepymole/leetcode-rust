#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut f = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m {
            if matrix[i][0] == '1' {
                f[i][0] = 1;
                ans = 1;
            }
        }
        for j in 1..n {
            if matrix[0][j] == '1' {
                f[0][j] = 1;
                ans = 1;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == '1' {
                    let m = f[i - 1][j].min(f[i][j - 1]);
                    f[i][j] = if matrix[i - m][j - m] == '1' {
                        m + 1
                    } else {
                        m
                    };
                    ans = ans.max((f[i][j] * f[i][j]) as i32);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_square() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![vec!['0', '1'], vec!['1', '0']]),
            1
        );
    }
}
