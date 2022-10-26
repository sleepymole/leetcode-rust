#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut k = k as usize;
        if k * 2 > prices.len() {
            k = prices.len() / 2;
        }
        let mut buy = vec![-prices[0]; k + 1];
        let mut sell = vec![0; k + 1];
        for p in prices.iter().skip(1) {
            for i in 1..=k {
                buy[i] = buy[i].max(sell[i - 1] - p);
                sell[i] = sell[i].max(buy[i] + p);
            }
        }
        sell[k]
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
