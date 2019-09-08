#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut m = HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X', 10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);
        m.insert('\x00', 10000);

        let mut num = 0;
        let mut prev = '\x00';
        for c in s.chars() {
            num += m.get(&c).unwrap();
            if m.get(&prev).unwrap() < m.get(&c).unwrap() {
                num -= 2 * m.get(&prev).unwrap();
            }
            prev = c;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_owned()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_owned()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_owned()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
