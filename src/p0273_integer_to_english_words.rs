#![allow(dead_code)]
pub struct Solution {}

static BLEW20: [&str; 20] = [
    "Zero",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];

static TENS: [&str; 10] = [
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num < 20 {
            return BLEW20[num as usize].to_owned();
        }
        if num < 100 {
            let mut s = String::new();
            s.push_str(TENS[(num / 10) as usize]);
            if num % 10 != 0 {
                s.push(' ');
                s.push_str(BLEW20[(num % 10) as usize]);
            }
            return s;
        }
        if num < 1000 {
            let mut s = String::new();
            s.push_str(Solution::number_to_words(num / 100).as_str());
            s.push_str(" Hundred");
            if num % 100 != 0 {
                s.push(' ');
                s.push_str(Solution::number_to_words(num % 100).as_str());
            }
            return s;
        }
        let mut num = num;
        let mut base = 1_000_000_000;
        for u in &[" Billion", " Million", " Thousand"] {
            if num >= base {
                let mut s = String::new();
                s.push_str(Solution::number_to_words(num / base).as_str());
                s.push_str(u);
                num %= base;
                if num > 0 {
                    s.push(' ');
                    s.push_str(Solution::number_to_words(num).as_str());
                }
                return s;
            }
            base /= 1000;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_words() {
        assert_eq!(
            Solution::number_to_words(123),
            "One Hundred Twenty Three".to_owned()
        );
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five".to_owned()
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_owned()
        );
        assert_eq!(
            Solution::number_to_words(1234567891),
            "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One".to_owned()
        );
    }
}
