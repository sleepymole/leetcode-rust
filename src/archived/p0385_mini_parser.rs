#![allow(dead_code)]
pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

use std::iter::Peekable;
use std::str::Chars;

impl Solution {
    fn parse_num(stream: &mut Peekable<Chars>) -> i32 {
        let mut num = 0;
        let mut sign = 1;
        if let Some('-') = stream.peek() {
            sign = -1;
            stream.next();
        }
        while let Some(&c) = stream.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            num = num * 10 + c as i32 - '0' as i32;
            stream.next();
        }
        sign * num
    }

    fn parse_list(stream: &mut Peekable<Chars>) -> Vec<NestedInteger> {
        if let Some(']') = stream.peek() {
            return Vec::new();
        }
        let mut list = vec![Solution::parse_int(stream)];
        while let Some(&c) = stream.peek() {
            if c == ']' {
                break;
            };
            assert_eq!(Some(','), stream.next());
            list.push(Solution::parse_int(stream));
        }
        list
    }

    fn parse_int(stream: &mut Peekable<Chars>) -> NestedInteger {
        if let Some('[') = stream.peek() {
            assert_eq!(Some('['), stream.next());
            let list = Solution::parse_list(stream);
            assert_eq!(Some(']'), stream.next());
            NestedInteger::List(list)
        } else {
            NestedInteger::Int(Solution::parse_num(stream))
        }
    }

    pub fn deserialize(s: String) -> NestedInteger {
        let mut stream = s.chars().peekable();
        Solution::parse_int(&mut stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        assert_eq!(
            Solution::deserialize("324".to_owned()),
            NestedInteger::Int(324)
        );
        assert_eq!(
            Solution::deserialize("[123,[456,[789]]]".to_owned()),
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::Int(456),
                    NestedInteger::List(vec![NestedInteger::Int(789)])
                ])
            ])
        );
    }
}
