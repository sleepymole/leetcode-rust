#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn is_repeat(chars: &Vec<char>, k: usize) -> bool {
        if k == chars.len() {
            return false;
        }
        for i in 0..chars.len() / k {
            for j in 0..k {
                if chars[i * k + j] != chars[j] {
                    return false;
                }
            }
        }
        true
    }

    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        for i in 1..=n {
            if i * i > n {
                break;
            }
            if n % i != 0 {
                continue;
            }
            if Solution::is_repeat(&chars, i) || Solution::is_repeat(&chars, n / i) {
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
    fn test_repeated_substring_pattern() {
        assert!(Solution::repeated_substring_pattern("abab".to_owned()));
        assert!(!Solution::repeated_substring_pattern("aba".to_owned()));
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_owned()
        ));
        assert!(Solution::repeated_substring_pattern(
            "babbabbabbabbab".to_owned()
        ));
    }
}
