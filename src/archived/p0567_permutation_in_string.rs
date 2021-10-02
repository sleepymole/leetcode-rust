#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut m1 = HashMap::new();
        for c in s1.chars() {
            *m1.entry(c).or_insert(0) += 1;
        }
        let keys = m1.keys().count();
        let s2: Vec<char> = s2.chars().collect();
        let mut m2 = HashMap::new();
        let mut matched = 0;
        for i in 0..s1.len() {
            if !m1.contains_key(&s2[i]) {
                continue;
            }
            let v2 = m2.entry(s2[i]).or_insert(0);
            *v2 += 1;
            if let Some(&v1) = m1.get(&s2[i]) {
                if *v2 == v1 {
                    matched += 1;
                }
            }
        }
        if matched == keys {
            return true;
        }
        for i in s1.len()..s2.len() {
            if m1.contains_key(&s2[i]) {
                let v2 = m2.entry(s2[i]).or_insert(0);
                *v2 += 1;
                if let Some(&v1) = m1.get(&s2[i]) {
                    if *v2 == v1 {
                        matched += 1;
                    }
                }
            }
            if let Some(v2) = m2.get_mut(&s2[i - s1.len()]) {
                if let Some(&v1) = m1.get(&s2[i - s1.len()]) {
                    if *v2 == v1 {
                        matched -= 1;
                    }
                }
                *v2 -= 1;
            }
            if matched == keys {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        assert!(Solution::check_inclusion(
            "ab".to_owned(),
            "eidbaooo".to_owned()
        ));
        assert!(!Solution::check_inclusion(
            "ab".to_owned(),
            "eidboaoo".to_owned()
        ));
    }
}
