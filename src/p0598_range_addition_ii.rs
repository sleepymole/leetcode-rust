#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut x, mut y) = (m, n);
        for op in ops {
            x = x.min(op[0]);
            y = y.min(op[1]);
        }
        x * y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_count() {
        assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2]]), 4);
        assert_eq!(
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            ),
            4
        );
        assert_eq!(Solution::max_count(3, 3, vec![]), 9);
    }
}
