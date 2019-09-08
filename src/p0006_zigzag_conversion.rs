#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let n = ((num_rows - 1) * 2) as usize;
        let mut p = String::new();
        for i in 0..num_rows as usize {
            let mut j = i;
            while j < s.len() {
                p.push(chars[j]);
                if i > 0 && i + 1 < num_rows as usize && j + (n - 2 * i) < s.len() {
                    p.push(chars[j + (n - 2 * i)]);
                }
                j += n
            }
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
    }
}
