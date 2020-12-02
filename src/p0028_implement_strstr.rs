#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let needle = needle.as_bytes();
        let mut fail: Vec<i32> = Vec::new();
        fail.resize(needle.len() + 1, -1);
        let mut i = 0;
        let mut j: i32 = -1;
        while i < needle.len() {
            if j != -1 && i < needle.len() && needle[i] == needle[j as usize] {
                fail[i] = fail[j as usize];
            } else {
                fail[i] = j;
            }
            while j != -1 && needle[i] != needle[j as usize] {
                j = fail[j as usize];
            }
            i += 1;
            j += 1;
        }
        fail[i] = j;

        let haystack = haystack.as_bytes();
        let mut j: i32 = 0;
        for i in 0..haystack.len() {
            while j != -1 && haystack[i] != needle[j as usize] {
                j = fail[j as usize];
            }
            j += 1;
            if j == needle.len() as i32 {
                return (i - needle.len() + 1) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(Solution::str_str("hello".to_owned(), "ll".to_owned()), 2);
        assert_eq!(Solution::str_str("aaaaa".to_owned(), "bba".to_owned()), -1);
        assert_eq!(Solution::str_str("".to_owned(), "".to_owned()), 0);
    }
}
