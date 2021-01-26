#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut f = vec![vec![0; k + 1]; prices.len() + 1];
        for i in 1..=prices.len() {
            f[i] = f[i - 1].clone();
            let mut min = prices[i - 1];
            for j in (1..i).rev() {
                min = min.min(prices[j - 1]);
                for l in 1..=k {
                    f[i][l] = f[i][l].max(f[j - 1][l - 1] + prices[i - 1] - min);
                }
            }
        }
        f[prices.len()][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
