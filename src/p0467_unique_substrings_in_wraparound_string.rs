#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut m = HashMap::new();
        let p: Vec<char> = p.chars().collect();
        let mut i = 0;
        while i < p.len() {
            let mut j = i + 1;
            while j < p.len() && (p[j] as i32 - p[j - 1] as i32 + 26) % 26 == 1 {
                j += 1;
            }

            for a in i..j.min(i + 26) {
                let k = m.entry(&p[a]).or_insert(0);
                *k = (*k).max(j - a);
            }
            i = j;
        }
        let mut res = 0;
        for v in m.values() {
            res += v;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substring_in_wrapround_string() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string("a".to_owned()),
            1
        );
        assert_eq!(
            Solution::find_substring_in_wrapround_string("cac".to_owned()),
            2
        );
        assert_eq!(
            Solution::find_substring_in_wrapround_string("zab".to_owned()),
            6
        );
    }
}
