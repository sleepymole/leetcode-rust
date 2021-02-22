#![allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    fn compute(chars: &mut VecDeque<char>) -> i32 {
        let mut ans = 0;
        while !chars.is_empty() && chars[0] != ')' {
            let mut sign = 1;
            while chars[0] == '+' || chars[0] == '-' {
                if chars[0] == '-' {
                    sign = -sign;
                }
                chars.pop_front();
            }
            if chars[0] == '(' {
                chars.pop_front();
                ans += sign * Solution::compute(chars);
                chars.pop_front();
            } else {
                let mut x = 0;
                while let Some(&c) = chars.front() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    x = x * 10 + c as i32 - '0' as i32;
                    chars.pop_front();
                }
                ans += sign * x;
            }
        }
        ans
    }

    pub fn calculate(s: String) -> i32 {
        let mut chars: VecDeque<char> = s.chars().filter(|&c| c != ' ').collect();
        Solution::compute(&mut chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(Solution::calculate("1 + 1".to_owned()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_owned()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("(7)-(0)+(4)".to_owned()), 11);
    }
}
