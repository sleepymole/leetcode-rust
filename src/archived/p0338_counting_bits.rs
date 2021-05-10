#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut bits = vec![0];
        let mut prev_bits = 0;
        for i in 1..=num {
            if (i & 1) == 0 {
                prev_bits -= bits[((i ^ (i - 1)) & (i - 1)) as usize];
            }
            bits.push(prev_bits + 1);
            prev_bits += 1;
        }
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
