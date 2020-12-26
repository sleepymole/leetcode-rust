#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut f = vec![0; prices.len()];
        let mut min = prices[0];
        for i in 1..prices.len() {
            f[i] = f[i - 1].max(prices[i] - min);
            min = min.min(prices[i]);
        }
        let mut ans = f[f.len() - 1];
        let mut max = prices[prices.len() - 1];
        for i in (1..prices.len() - 1).rev() {
            ans = ans.max(f[i - 1] + max - prices[i]);
            max = max.max(prices[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
