#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            carry += digits[i];
            digits[i] = carry % 10;
            carry /= 10;
        }
        if carry > 0 {
            let mut ans = vec![carry];
            ans.append(&mut digits);
            digits = ans
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    }
}
