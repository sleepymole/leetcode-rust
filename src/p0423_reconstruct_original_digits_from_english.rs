#![allow(dead_code)]
pub struct Solution;

use std::char;
use std::collections::HashMap;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut m = HashMap::new();
        for c in s.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        let words = vec![
            "zero".to_owned(),
            "one".to_owned(),
            "two".to_owned(),
            "three".to_owned(),
            "four".to_owned(),
            "five".to_owned(),
            "six".to_owned(),
            "seven".to_owned(),
            "eight".to_owned(),
            "nine".to_owned(),
        ];
        let mut cindex: HashMap<char, Vec<usize>> = HashMap::new();
        for i in 0..10 {
            for c in words[i].chars() {
                cindex.entry(c).or_insert(Vec::new()).push(i);
            }
        }
        let mut nums = Vec::new();
        while !cindex.is_empty() {
            let mut u = '\x00';
            let mut x = 10;
            for (&c, v) in &cindex {
                if v.len() == 1 {
                    u = c;
                    x = v[0];
                }
            }
            assert!(x < 10);
            for c in words[x].chars() {
                if let Some(v) = cindex.get_mut(&c) {
                    for i in 0..v.len() {
                        if v[i] == x {
                            v.swap_remove(i);
                            if v.is_empty() {
                                cindex.remove(&c);
                            }
                            break;
                        }
                    }
                }
            }
            let count = m.get(&u).cloned().unwrap_or_default();
            for c in words[x].chars() {
                if let Some(v) = m.get_mut(&c) {
                    *v -= count;
                }
            }
            nums.resize(
                nums.len() + count,
                char::from_u32('0' as u32 + x as u32).unwrap(),
            );
        }
        nums.sort_unstable();
        nums.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_original_digits() {
        assert_eq!(
            Solution::original_digits("owoztneoer".to_owned()),
            "012".to_owned()
        );
        assert_eq!(
            Solution::original_digits("fviefuro".to_owned()),
            "45".to_owned()
        );
    }
}
