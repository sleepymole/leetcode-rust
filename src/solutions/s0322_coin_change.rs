#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let huge = 0x3f3f3f3f;
        let n = amount as usize;
        let mut f = vec![huge; n + 1];
        f[0] = 0;
        for c in coins {
            for j in c as usize..=n {
                f[j] = f[j].min(f[j - c as usize] + 1);
            }
        }
        if f[n] < huge {
            f[n]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
        assert_eq!(Solution::coin_change(vec![1], 2), 2);
    }
}
