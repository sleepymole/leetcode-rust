#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() <= 1 {
            return true;
        }
        let mut chars = word.chars();
        if chars.next().unwrap().is_ascii_uppercase() && chars.next().unwrap().is_ascii_uppercase()
        {
            chars.all(|c| c.is_ascii_uppercase())
        } else {
            chars.all(|c| !c.is_ascii_uppercase())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_capital_use() {
        assert!(Solution::detect_capital_use("USA".to_owned()));
        assert!(!Solution::detect_capital_use("FlaG".to_owned()));
    }
}
