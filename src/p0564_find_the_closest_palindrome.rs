#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn nearest_palindromic(s: String) -> String {
        let n: i64 = s.parse().unwrap();
        if n < 10 {
            return if n == 0 {
                "1".to_owned()
            } else {
                (n - 1).to_string()
            };
        }

        let s: Vec<char> = s.chars().collect();
        let mut nums = Vec::new();
        nums.push(10i64.pow(s.len() as u32) + 1);
        nums.push(10i64.pow(s.len() as u32 - 1) - 1);
        let make_pali = |x: i64| {
            let mut rx = 0;
            let mut t = x;
            while t > 0 {
                rx = rx * 10 + t % 10;
                t /= 10;
            }
            if s.len() % 2 == 0 {
                x * 10i64.pow(s.len() as u32 / 2) + rx
            } else {
                x / 10 * 10i64.pow((s.len() as u32 + 1) / 2) + rx
            }
        };
        let a = n / 10i64.pow(s.len() as u32 / 2);
        nums.push(make_pali(a - 1));
        nums.push(make_pali(a));
        nums.push(make_pali(a + 1));
        nums.sort_unstable_by(|x, y| ((x - n).abs(), x).cmp(&((y - n).abs(), y)));
        nums.into_iter().find(|&v| v != n).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_palindromic() {
        assert_eq!(
            Solution::nearest_palindromic("123".to_owned()),
            "121".to_owned()
        );
        assert_eq!(
            Solution::nearest_palindromic("1".to_owned()),
            "0".to_owned()
        );
    }
}
