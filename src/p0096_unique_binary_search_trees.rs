#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut f = Vec::new();
        f.resize((n + 1) as usize, 0);
        f[0] = 1;
        for i in 1..=n as usize {
            for j in 0..i {
                f[i] += f[j] * f[i - 1 - j];
            }
        }
        f[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_trees() {
        assert_eq!(Solution::num_trees(1), 1);
        assert_eq!(Solution::num_trees(2), 2);
        assert_eq!(Solution::num_trees(3), 5);
    }
}
