#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = String::new();
        for w in s.split_whitespace() {
            let rw: String = w.chars().rev().collect();
            if !res.is_empty() {
                res.push(' ');
            }
            res.push_str(&rw);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_owned()),
            "s'teL ekat edoCteeL tsetnoc".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("God Ding".to_owned()),
            "doG gniD".to_owned()
        );
    }
}
