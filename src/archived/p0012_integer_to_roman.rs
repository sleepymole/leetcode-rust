#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let s = vec![vec!['I', 'V'], vec!['X', 'L'], vec!['C', 'D'], vec!['M']];
        let mut num = num;
        let mut base = 1000;
        let mut roman = String::new();
        for i in (0..4).rev() {
            let mut d = num / base;
            num %= base;
            base /= 10;
            if d == 4 {
                roman.push(s[i][0]);
                roman.push(s[i][1]);
                continue;
            }
            if d == 9 {
                roman.push(s[i][0]);
                roman.push(s[i + 1][0]);
                continue;
            }
            if d >= 5 {
                roman.push(s[i][1]);
            }
            d %= 5;
            for _j in 0..d {
                roman.push(s[i][0]);
            }
        }
        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
