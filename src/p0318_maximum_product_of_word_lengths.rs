#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut charsets = vec![0; words.len()];
        for i in 0..words.len() {
            for c in words[i].chars() {
                charsets[i] |= 1 << (c as i32 - 'a' as i32);
            }
        }
        let mut ans = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if (charsets[i] & charsets[j]) == 0 && words[i].len() * words[j].len() > ans {
                    ans = words[i].len() * words[j].len();
                }
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(
            Solution::max_product(vec![
                "abcw".to_owned(),
                "baz".to_owned(),
                "foo".to_owned(),
                "bar".to_owned(),
                "xtfn".to_owned(),
                "abcdef".to_owned()
            ]),
            16
        );
        assert_eq!(
            Solution::max_product(vec![
                "a".to_owned(),
                "ab".to_owned(),
                "abc".to_owned(),
                "d".to_owned(),
                "cd".to_owned(),
                "bcd".to_owned(),
                "abcd".to_owned()
            ]),
            4
        );
        assert_eq!(
            Solution::max_product(vec![
                "a".to_owned(),
                "aa".to_owned(),
                "aaa".to_owned(),
                "aaaa".to_owned()
            ]),
            0
        );
    }
}
