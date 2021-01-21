#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut ans = 0;
        for c in s.chars() {
            let x = c as i32 - 'A' as i32 + 1;
            ans = ans * 26 + x;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(Solution::title_to_number("A".to_owned()), 1);
        assert_eq!(Solution::title_to_number("AB".to_owned()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_owned()), 701);
    }
}
