#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let secret: Vec<char> = secret.chars().collect();
        let guess: Vec<char> = guess.chars().collect();
        let mut m = HashMap::new();
        let mut bulls = 0;
        let mut cows = 0;
        for i in 0..guess.len() {
            if guess[i] == secret[i] {
                bulls += 1;
            } else {
                *m.entry(secret[i]).or_insert(0) += 1;
            }
        }
        for i in 0..guess.len() {
            if guess[i] == secret[i] {
                continue;
            }
            if let Some(n) = m.get_mut(&guess[i]) {
                if *n > 0 {
                    cows += 1;
                    *n -= 1;
                }
            }
        }
        format!("{bulls}A{cows}B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hint() {
        assert_eq!(
            Solution::get_hint("1807".to_owned(), "7810".to_owned()),
            "1A3B".to_owned()
        );
        assert_eq!(
            Solution::get_hint("1123".to_owned(), "0111".to_owned()),
            "1A1B".to_owned()
        );
        assert_eq!(
            Solution::get_hint("1".to_owned(), "0".to_owned()),
            "0A0B".to_owned()
        );
        assert_eq!(
            Solution::get_hint("1".to_owned(), "1".to_owned()),
            "1A0B".to_owned()
        );
    }
}
