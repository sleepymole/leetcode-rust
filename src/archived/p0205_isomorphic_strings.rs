#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        let mut t = t.chars();
        for a in s.chars() {
            let b = t.next().unwrap();
            if let Some(&c) = m1.get(&a) {
                if b != c {
                    return false;
                }
            } else {
                if m2.contains_key(&b) {
                    return false;
                }
                m1.insert(a, b);
                m2.insert(b, a);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_owned(), "add".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_owned(), "title".to_owned()),
            true
        );
    }
}
