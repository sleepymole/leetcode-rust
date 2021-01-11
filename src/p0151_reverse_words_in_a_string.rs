#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut ans = String::new();
        let mut i = 0;
        while i < s.len() {
            if s[i] == ' ' {
                i += 1;
                continue;
            }
            let mut j = i;
            while j + 1 < s.len() && s[j + 1] != ' ' {
                j += 1;
            }
            if !ans.is_empty() {
                ans.push(' ');
            }
            for k in (i..=j).rev() {
                ans.push(s[k]);
            }
            i = j + 1;
        }
        ans.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_owned()),
            "blue is sky the"
        );
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_owned()),
            "world hello".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("a good   example".to_owned()),
            "example good a".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("  Bob    Loves  Alice   ".to_owned()),
            "Alice Loves Bob".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("Alice does not even like bob".to_owned()),
            "bob like even not does Alice".to_owned()
        );
    }
}
