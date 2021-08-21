#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let k = k as usize;
        let mut i = 0;
        while i < s.len() {
            let r = k.min(s.len() - i);
            let (mut a, mut b) = (i, i + r - 1);
            while a < b {
                s.swap(a, b);
                a += 1;
                b -= 1;
            }
            i += k * 2;
        }
        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_str() {
        assert_eq!(
            Solution::reverse_str("abcdefg".to_owned(), 2),
            "bacdfeg".to_owned()
        );
        assert_eq!(
            Solution::reverse_str("abcd".to_owned(), 2),
            "bacd".to_owned()
        );
    }
}
