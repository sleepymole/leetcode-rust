#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut f = vec![vec![0; n]; m];
        if dungeon[m - 1][n - 1] < 1 {
            f[m - 1][n - 1] = 1 - dungeon[m - 1][n - 1];
        }
        for i in (0..m - 1).rev() {
            if dungeon[i][n - 1] < 1 {
                f[i][n - 1] = 1 - dungeon[i][n - 1];
                if f[i + 1][n - 1] > 1 {
                    f[i][n - 1] += f[i + 1][n - 1] - 1;
                }
            } else if dungeon[i][n - 1] < f[i + 1][n - 1] {
                f[i][n - 1] = f[i + 1][n - 1] - dungeon[i][n - 1];
            }
        }
        for j in (0..n - 1).rev() {
            if dungeon[m - 1][j] < 1 {
                f[m - 1][j] = 1 - dungeon[m - 1][j];
                if f[m - 1][j + 1] > 1 {
                    f[m - 1][j] += f[m - 1][j + 1] - 1;
                }
            } else if dungeon[m - 1][j] < f[m - 1][j + 1] {
                f[m - 1][j] = f[m - 1][j + 1] - dungeon[m - 1][j];
            }
        }
        for i in (0..m - 1).rev() {
            for j in (0..n - 1).rev() {
                let min = f[i + 1][j].min(f[i][j + 1]);
                if dungeon[i][j] < 1 {
                    f[i][j] = 1 - dungeon[i][j];
                    if min > 1 {
                        f[i][j] += min - 1;
                    }
                } else if dungeon[i][j] < min {
                    f[i][j] = min - dungeon[i][j]
                }
            }
        }
        if f[0][0] > 0 {
            f[0][0]
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_minimum_hp() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5]
            ]),
            7
        );
    }
}
