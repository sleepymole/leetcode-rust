#![allow(dead_code)]
pub struct Solution;

use std::str::Chars;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }
        let mut strs: Vec<Chars> = strs.iter().map(|s| s.chars()).collect();
        let mut prefix = String::new();
        loop {
            let mut ch: Option<char> = None;
            for s in strs.iter_mut() {
                let mut cur = s.next();
                if cur.is_none() {
                    return prefix;
                }
                match ch {
                    Some(last) => {
                        if last != cur.unwrap() {
                            return prefix;
                        }
                    }
                    None => {
                        ch = cur.take();
                    }
                }
            }
            prefix.push(ch.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ]),
            "fl".to_owned()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            "".to_owned()
        )
    }
}
