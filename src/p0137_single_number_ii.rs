#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bits = [0; 32];
        for x in nums.iter() {
            for i in 0..32 {
                if x & (1 << i) != 0 {
                    bits[i] = (bits[i] + 1) % 3;
                }
            }
        }
        let mut ans = 0;
        for i in (0..32).rev() {
            ans = ans * 2 + bits[i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
