#![allow(dead_code)]
pub struct Solution {}

use std::cmp::Reverse;
use std::collections::BTreeMap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut p = Vec::new();
        for i in 0..n {
            p.push((capital[i], profits[i]));
        }
        p.sort_unstable();
        let mut m = BTreeMap::new();
        let mut pi = 0;
        let mut w = w;
        for _ in 0..k {
            while pi < p.len() && p[pi].0 <= w {
                *m.entry(Reverse(p[pi].1)).or_insert(0) += 1;
                pi += 1;
            }
            if m.is_empty() {
                break;
            }

            if let Some((&Reverse(p), cnt)) = m.range_mut(Reverse(i32::MAX)..).next() {
                w += p;
                *cnt -= 1;
                if *cnt == 0 {
                    m.remove(&Reverse(p));
                }
            }
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximized_capital() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }
}
