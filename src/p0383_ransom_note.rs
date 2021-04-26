#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut m = HashMap::new();
        for c in magazine.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        for c in ransom_note.chars() {
            *m.entry(c).or_insert(0) -= 1;
            if *m.get(&c).unwrap() < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        assert_eq!(
            Solution::can_construct("a".to_owned(), "b".to_owned()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "ab".to_owned()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "aab".to_owned()),
            true
        );
    }
}
