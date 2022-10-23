#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let s: Vec<char> = s.trim().chars().collect();
        if s.is_empty() {
            return false;
        }
        let mut epos = s.len();
        for i in 0..s.len() {
            if s[i] == 'e' {
                epos = i;
                break;
            }
        }
        let mut cur_pos = 0;
        if cur_pos < epos {
            if s[cur_pos] == '+' || s[cur_pos] == '-' {
                cur_pos += 1;
            }
            if cur_pos == epos {
                return false;
            }
            let mut dot_found = false;
            for i in cur_pos..epos {
                if !s[i].is_ascii_digit() {
                    if s[i] != '.' || dot_found {
                        return false;
                    }
                    dot_found = true;
                }
            }
            if dot_found && cur_pos + 1 == epos {
                return false;
            }
        }
        if epos < s.len() {
            if epos == 0 || epos + 1 == s.len() {
                return false;
            }
            epos += 1;
            if s[epos] == '+' || s[epos] == '-' {
                epos += 1;
            }
            if epos == s.len() {
                return false;
            }
            for i in epos..s.len() {
                if !s[i].is_ascii_digit() {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        assert!(Solution::is_number("0".to_owned()));
        assert!(Solution::is_number("3".to_owned()));
        assert!(!Solution::is_number(".".to_owned()));
        assert!(!Solution::is_number("e9".to_owned()));
        assert!(Solution::is_number("1 ".to_owned()));
        assert!(Solution::is_number("-1.".to_owned()));
        assert!(Solution::is_number(" 005047e+6".to_owned()));
        assert!(Solution::is_number("2e0".to_owned()));
        assert!(!Solution::is_number("e".to_owned()));
    }
}
