#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stk.push(c);
                continue;
            }
            if stk.is_empty() {
                return false;
            }
            if let Some(x) = stk.pop() {
                if c == ')' && x != '(' || c == ']' && x != '[' || c == '}' && x != '{' {
                    return false;
                }
            }
        }
        stk.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_owned()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_owned()), true);
        assert_eq!(Solution::is_valid("(]".to_owned()), false);
    }
}
