#![allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    fn muldiv(chars: &mut VecDeque<char>) -> i32 {
        let mut ans = 1;
        chars.push_front('*');
        while !chars.is_empty() && chars[0] != '+' && chars[0] != '-' {
            let is_mul = match chars.pop_front() {
                Some('*') => true,
                Some('/') => false,
                _ => unreachable!(),
            };
            let mut sign = 1;
            while chars[0] == '+' || chars[0] == '-' {
                if chars[0] == '-' {
                    sign = -sign;
                }
                chars.pop_front();
            }

            let mut x = 0;
            while let Some(&c) = chars.front() {
                if !c.is_ascii_digit() {
                    break;
                }
                x = x * 10 + c as i32 - '0' as i32;
                chars.pop_front();
            }
            if is_mul {
                ans *= sign * x;
            } else {
                ans /= sign * x;
            }
        }
        ans
    }

    fn addsub(chars: &mut VecDeque<char>) -> i32 {
        let mut ans = 0;
        while !chars.is_empty() {
            let mut is_add = true;
            if chars[0] == '+' || chars[0] == '-' {
                if chars[0] == '-' {
                    is_add = false;
                }
                chars.pop_front();
            }
            if is_add {
                ans += Solution::muldiv(chars);
            } else {
                ans -= Solution::muldiv(chars);
            }
        }
        ans
    }

    pub fn calculate(s: String) -> i32 {
        let mut chars = s.chars().filter(|&c| c != ' ').collect();
        Solution::addsub(&mut chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(Solution::calculate("3+2*2".to_owned()), 7);
        assert_eq!(Solution::calculate(" 3/2 ".to_owned()), 1);
        assert_eq!(Solution::calculate(" 3+5 / 2 ".to_owned()), 5);
    }
}
