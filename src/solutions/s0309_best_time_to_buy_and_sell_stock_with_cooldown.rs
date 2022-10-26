#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut f = vec![0; prices.len()];
        let mut g = vec![0; prices.len()];
        for i in 1..prices.len() {
            f[i] = f[i - 1].max(0) + prices[i] - prices[i - 1];
            if i >= 3 {
                f[i] = f[i].max(g[i - 3] + prices[i] - prices[i - 1]);
            }
            g[i] = g[i - 1].max(f[i]);
        }
        g[prices.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 4]), 3);
        assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 4, 7]), 6);
    }
}
