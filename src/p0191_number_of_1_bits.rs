#![allow(dead_code)]
#![allow(non_snake_case)]
pub struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            if n & (1 << i) > 0 {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hammingWeight() {
        assert_eq!(Solution::hammingWeight(11), 3);
        assert_eq!(Solution::hammingWeight(128), 1);
        assert_eq!(Solution::hammingWeight(4294967294), 31);
    }
}
