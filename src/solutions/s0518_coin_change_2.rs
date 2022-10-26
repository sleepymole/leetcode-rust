#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut f = vec![0; amount as usize + 1];
        f[0] = 1;
        for c in coins {
            for a in (0..=amount).rev() {
                let mut m = c;
                while m <= a {
                    f[a as usize] += f[(a - m) as usize];
                    m += c;
                }
            }
        }
        f[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
        assert_eq!(Solution::change(3, vec![2]), 0);
        assert_eq!(Solution::change(10, vec![10]), 1);
    }
}
