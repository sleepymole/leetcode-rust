#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<char> = a.chars().rev().collect();
        let b: Vec<char> = b.chars().rev().collect();
        let mut c = Vec::new();
        let mut overflow = 0;
        for i in 0..a.len().min(b.len()) {
            let x = a[i] as i32 - '0' as i32 + b[i] as i32 - '0' as i32 + overflow;
            if x % 2 == 0 {
                c.push('0');
            } else if x % 2 == 1 {
                c.push('1');
            }
            overflow = x / 2;
        }
        for i in a.len()..b.len() {
            let x = b[i] as i32 - '0' as i32 + overflow;
            if x % 2 == 0 {
                c.push('0');
            } else if x % 2 == 1 {
                c.push('1');
            }
            overflow = x / 2;
        }
        for i in b.len()..a.len() {
            let x = a[i] as i32 - '0' as i32 + overflow;
            if x % 2 == 0 {
                c.push('0');
            } else if x % 2 == 1 {
                c.push('1');
            }
            overflow = x / 2;
        }
        if overflow > 0 {
            c.push('1');
        }
        c.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(
            Solution::add_binary("11".to_owned(), "1".to_owned()),
            "100".to_owned(),
        );
        assert_eq!(
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
            "10101".to_owned(),
        );
        assert_eq!(
            Solution::add_binary("1111".to_owned(), "1111".to_owned()),
            "11110".to_owned(),
        );
    }
}
