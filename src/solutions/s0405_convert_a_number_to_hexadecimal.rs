#![allow(dead_code)]
#![allow(clippy::wrong_self_convention)]
pub struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        format!("{num:x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hex() {
        assert_eq!(Solution::to_hex(26), "1a".to_owned());
        assert_eq!(Solution::to_hex(-1), "ffffffff".to_owned());
    }
}
