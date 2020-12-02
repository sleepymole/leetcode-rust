#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut stk = Vec::new();
        let mut dp: Vec<usize> = Vec::new();
        dp.resize(s.len(), 0);
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stk.push(i);
            } else {
                if let Some(x) = stk.pop() {
                    dp[i] = i - x + 1;
                    if x > 0 {
                        dp[i] += dp[x - 1];
                    }
                    ans = usize::max(ans, dp[i]);
                }
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_owned()), 0);
        assert_eq!(
            Solution::longest_valid_parentheses("(((((()()".to_owned()),
            4
        );
    }
}
