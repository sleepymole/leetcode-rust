#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut longest = 0;
        let mut abs_len = 0;
        let mut stack = Vec::new();
        let mut chars = input.chars();
        loop {
            let mut depth = 0;
            let mut len = 0;
            let mut has_dot = false;
            for c in chars.by_ref() {
                if c == '\n' {
                    break;
                }
                if len == 0 && c == '\t' {
                    depth += 1;
                } else {
                    has_dot |= c == '.';
                    len += 1;
                }
            }
            if len == 0 {
                break;
            }
            while stack.len() > depth {
                if let Some(l) = stack.pop() {
                    abs_len -= l;
                }
            }
            stack.push(len);
            abs_len += len;
            if has_dot {
                longest = longest.max(abs_len + stack.len() - 1);
            }
        }
        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_longest_path() {
        assert_eq!(
            Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_owned()),
            20
        );
        assert_eq!(
            Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_owned()),
            32
        );
        assert_eq!(Solution::length_longest_path("a".to_owned()), 0);
        assert_eq!(
            Solution::length_longest_path("file1.txt\nfile2.txt\nlongfile.txt".to_owned()),
            12
        );
    }
}
