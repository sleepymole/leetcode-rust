#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s: Vec<char> = s.to_uppercase().chars().filter(|&c| c != '-').collect();
        let k = k as usize;
        let first = s.len() % k;
        let mut res = String::new();
        for i in 0..first {
            res.push(s[i]);
        }
        let mut i = first;
        while i < s.len() {
            if !res.is_empty() {
                res.push('-');
            }
            for j in i..i + k {
                res.push(s[j]);
            }
            i += k;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_key_formatting() {
        assert_eq!(
            Solution::license_key_formatting("5F3Z-2e-9-w".to_owned(), 4),
            "5F3Z-2E9W".to_owned()
        );
        assert_eq!(
            Solution::license_key_formatting("2-5g-3-J".to_owned(), 2),
            "2-5G-3J".to_owned()
        );
    }
}
