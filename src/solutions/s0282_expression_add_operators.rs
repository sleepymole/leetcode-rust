#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn value(item: &[char]) -> i64 {
        let (mut r, mut x) = (1, 0);
        for &c in item {
            if c == '*' {
                r *= x;
                x = 0;
            } else {
                x = x * 10 + c as i64 - '0' as i64;
            }
        }
        r *= x;
        r
    }

    fn validate(expr: &[char], target: i32) -> bool {
        let mut r = 0;
        let mut i = 0;
        let mut sign = 1;
        while i < expr.len() {
            let mut j = i;
            while j < expr.len() && expr[j] != '+' && expr[j] != '-' {
                if expr[j] == '0'
                    && (j == i || !expr[j - 1].is_ascii_digit())
                    && j + 1 < expr.len()
                    && expr[j + 1].is_ascii_digit()
                {
                    return false;
                }
                j += 1;
            }
            r += sign * Solution::value(&expr[i..j]);
            sign = if j < expr.len() && expr[j] == '+' {
                1
            } else {
                -1
            };
            i = j + 1;
        }
        r == target as i64
    }

    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        if num.is_empty() {
            return vec![];
        }
        let mut ans = Vec::new();
        let n = num.len();
        for i in 0..(1 << (2 * (n - 1))) {
            let mut expr = vec![num.chars().next().unwrap()];
            for (j, c) in num.chars().skip(1).enumerate() {
                if let Some(op) = match (i & (1 << (2 * j)) > 0, i & (1 << (2 * j + 1)) > 0) {
                    (false, false) => None,
                    (false, true) => Some('+'),
                    (true, false) => Some('-'),
                    (true, true) => Some('*'),
                } {
                    expr.push(op);
                }
                expr.push(c);
            }
            if Solution::validate(&expr, target) {
                ans.push(expr.iter().collect());
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut ss: Vec<String>) -> Vec<String> {
        ss.sort_unstable();
        ss
    }

    #[test]
    fn test_add_operators() {
        assert_eq!(
            sorted(Solution::add_operators("123".to_owned(), 6)),
            sorted(vec!["1+2+3".to_owned(), "1*2*3".to_owned()])
        );
        assert_eq!(
            sorted(Solution::add_operators("232".to_owned(), 8)),
            sorted(vec!["2*3+2".to_owned(), "2+3*2".to_owned()])
        );
        assert_eq!(
            sorted(Solution::add_operators("105".to_owned(), 5)),
            sorted(vec!["1*0+5".to_owned(), "10-5".to_owned()])
        );
        assert_eq!(
            sorted(Solution::add_operators("00".to_owned(), 0)),
            sorted(vec!["0+0".to_owned(), "0-0".to_owned(), "0*0".to_owned()])
        );
        assert!(Solution::add_operators("3456237490".to_owned(), 9191).is_empty());
    }
}
