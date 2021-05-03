#![allow(dead_code)]
pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![vec![0; n + 1]; n + 1];
        for r in 2..=n {
            let mut k = r - 1;
            let mut q = VecDeque::new();
            q.push_back(r - 1);
            for l in (1..r).rev() {
                while k > l && f[l][k - 1] > f[k + 1][r] {
                    k -= 1;
                    while let Some(&a) = q.front() {
                        if a <= k {
                            break;
                        }
                        q.pop_front();
                    }
                }
                f[l][r] = f[l][k] + k + 1;
                if let Some(&a) = q.front() {
                    f[l][r] = f[l][r].min(f[a + 1][r] + a);
                }
                while let Some(&a) = q.back() {
                    if f[a + 1][r] + a < f[l][r] + l - 1 {
                        break;
                    }
                    q.pop_back();
                }
                q.push_back(l - 1);
            }
        }
        f[1][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_money_amount() {
        assert_eq!(Solution::get_money_amount(10), 16);
        assert_eq!(Solution::get_money_amount(1), 0);
        assert_eq!(Solution::get_money_amount(2), 1);
    }
}
