#![allow(dead_code)]
pub struct Solution;

use std::iter::Peekable;
use std::str::Chars;

impl Solution {
    fn parse_num(stream: &mut Peekable<Chars>) -> i32 {
        let mut num = 0;
        while let Some(&c) = stream.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            num = num * 10 + c as i32 - '0' as i32;
            stream.next();
        }
        num
    }

    fn parse_string(stream: &mut Peekable<Chars>) -> String {
        if let Some(']') = stream.peek() {
            return String::new();
        }
        let mut s = String::new();
        while let Some(&c) = stream.peek() {
            if c == ']' {
                break;
            };
            if c.is_ascii_digit() {
                let n = Solution::parse_num(stream);
                assert_eq!(Some('['), stream.next());
                let ss = Solution::parse_string(stream);
                assert_eq!(Some(']'), stream.next());
                for _ in 0..n {
                    s += &ss;
                }
            } else {
                s.push(c);
                stream.next();
            }
        }
        s
    }

    pub fn decode_string(s: String) -> String {
        let mut stream = s.chars().peekable();
        Solution::parse_string(&mut stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_owned()),
            "aaabcbc".to_owned()
        );
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_owned()),
            "accaccacc".to_owned()
        );
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_owned()),
            "abcabccdcdcdef".to_owned()
        );
    }
}
