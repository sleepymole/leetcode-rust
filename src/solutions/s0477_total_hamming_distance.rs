#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..=30 {
            let mut c0 = 0;
            let mut c1 = 0;
            for &x in &nums {
                if x & (1 << i) == 0 {
                    c0 += 1;
                } else {
                    c1 += 1;
                }
            }
            res += c0 * c1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_hamming_distance() {
        assert_eq!(Solution::total_hamming_distance(vec![4, 14, 2]), 6);
        assert_eq!(Solution::total_hamming_distance(vec![4, 14, 4]), 4);
    }
}
